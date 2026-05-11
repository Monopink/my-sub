import { createPublicKey, createVerify } from "node:crypto";
import type { JsonWebKey as NodeJsonWebKey } from "node:crypto";
import { NextResponse } from "next/server";

type JwtHeader = {
  alg?: string;
  kid?: string;
  typ?: string;
};

type JwtPayload = {
  iss?: string;
  aud?: string | string[];
  exp?: number;
  nbf?: number;
  iat?: number;
  sub?: string;
  email?: string;
};

type CertResponse = {
  keys?: Array<Record<string, unknown>>;
};

type CachedCerts = {
  expiresAtMs: number;
  keysByKid: Map<string, NodeJsonWebKey>;
};

let certCache: CachedCerts | null = null;
const CERT_CACHE_TTL_MS = 5 * 60 * 1000;

function fail(message: string, status = 401, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

function decodeBase64Url(input: string): string {
  const normalized = input.replace(/-/g, "+").replace(/_/g, "/");
  const padding = normalized.length % 4 === 0 ? "" : "=".repeat(4 - (normalized.length % 4));
  return Buffer.from(`${normalized}${padding}`, "base64").toString("utf8");
}

function parseJwt(token: string): { header: JwtHeader; payload: JwtPayload; signingInput: string; signature: Buffer } {
  const parts = token.split(".");
  if (parts.length !== 3) {
    throw new Error("invalid jwt format");
  }
  const [headerEncoded, payloadEncoded, signatureEncoded] = parts;
  const header = JSON.parse(decodeBase64Url(headerEncoded)) as JwtHeader;
  const payload = JSON.parse(decodeBase64Url(payloadEncoded)) as JwtPayload;
  const signature = Buffer.from(
    signatureEncoded.replace(/-/g, "+").replace(/_/g, "/"),
    "base64"
  );
  return {
    header,
    payload,
    signingInput: `${headerEncoded}.${payloadEncoded}`,
    signature,
  };
}

function expectedIssuer(teamDomain: string): string {
  const configured = (process.env.CF_ACCESS_ISSUER ?? "").trim();
  if (configured) {
    return configured;
  }
  return `https://${teamDomain}`;
}

function audienceMatches(payloadAud: JwtPayload["aud"], expectedAud: string): boolean {
  if (typeof payloadAud === "string") {
    return payloadAud === expectedAud;
  }
  if (Array.isArray(payloadAud)) {
    return payloadAud.includes(expectedAud);
  }
  return false;
}

async function fetchCertKeys(teamDomain: string): Promise<Map<string, NodeJsonWebKey>> {
  const now = Date.now();
  if (certCache && certCache.expiresAtMs > now) {
    return certCache.keysByKid;
  }

  const url = `https://${teamDomain}/cdn-cgi/access/certs`;
  const response = await fetch(url, { cache: "no-store" });
  if (!response.ok) {
    throw new Error(`failed to fetch certs: ${response.status}`);
  }
  const body = (await response.json()) as CertResponse;
  const keys = new Map<string, NodeJsonWebKey>();
  for (const key of body.keys ?? []) {
    const kid = typeof key.kid === "string" ? key.kid : "";
    if (!kid) {
      continue;
    }
    keys.set(kid, key as NodeJsonWebKey);
  }
  certCache = {
    expiresAtMs: now + CERT_CACHE_TTL_MS,
    keysByKid: keys,
  };
  return keys;
}

function verifySignature(
  signingInput: string,
  signature: Buffer,
  jwk: NodeJsonWebKey
): boolean {
  const publicKey = createPublicKey({ key: jwk, format: "jwk" });
  const verifier = createVerify("RSA-SHA256");
  verifier.update(signingInput);
  verifier.end();
  return verifier.verify(publicKey, signature);
}

export type AdminIdentity = {
  subject: string;
  email: string;
};

function tokenFromCookieHeader(cookieHeader: string | null): string | null {
  if (!cookieHeader) {
    return null;
  }
  const parts = cookieHeader.split(";");
  for (const part of parts) {
    const [name, ...rest] = part.trim().split("=");
    if (name !== "CF_Authorization") {
      continue;
    }
    const value = rest.join("=").trim();
    if (value) {
      return value;
    }
  }
  return null;
}

export async function requireAdminAccess(
  request: Request
): Promise<{ ok: true; identity: AdminIdentity } | { ok: false; response: NextResponse }> {
  const teamDomain = (process.env.CF_ACCESS_TEAM_DOMAIN ?? "").trim();
  const expectedAud = (process.env.CF_ACCESS_AUD ?? "").trim();

  if (!teamDomain || !expectedAud) {
    return {
      ok: false,
      response: fail(
        "admin auth not configured",
        500,
        "CF_ACCESS_TEAM_DOMAIN and CF_ACCESS_AUD are required"
      ),
    };
  }

  const headerToken = request.headers.get("cf-access-jwt-assertion")?.trim();
  const cookieToken = tokenFromCookieHeader(request.headers.get("cookie"));
  const token = headerToken || cookieToken;
  if (!token) {
    return { ok: false, response: fail("missing access token", 401) };
  }

  try {
    const parsed = parseJwt(token);
    if (parsed.header.alg !== "RS256") {
      return { ok: false, response: fail("invalid token algorithm", 401) };
    }
    if (!parsed.header.kid) {
      return { ok: false, response: fail("missing token kid", 401) };
    }

    const keys = await fetchCertKeys(teamDomain);
    const key = keys.get(parsed.header.kid);
    if (!key) {
      return { ok: false, response: fail("unknown token kid", 401) };
    }
    if (!verifySignature(parsed.signingInput, parsed.signature, key)) {
      return { ok: false, response: fail("invalid token signature", 401) };
    }

    const iss = parsed.payload.iss ?? "";
    if (iss !== expectedIssuer(teamDomain)) {
      return { ok: false, response: fail("invalid token issuer", 401) };
    }
    if (!audienceMatches(parsed.payload.aud, expectedAud)) {
      return { ok: false, response: fail("invalid token audience", 401) };
    }

    const nowSec = Math.floor(Date.now() / 1000);
    if (typeof parsed.payload.exp === "number" && parsed.payload.exp < nowSec) {
      return { ok: false, response: fail("token expired", 401) };
    }
    if (typeof parsed.payload.nbf === "number" && parsed.payload.nbf > nowSec) {
      return { ok: false, response: fail("token not active", 401) };
    }

    return {
      ok: true,
      identity: {
        subject: parsed.payload.sub ?? "unknown",
        email: parsed.payload.email ?? "unknown",
      },
    };
  } catch (error) {
    return { ok: false, response: fail("token verification failed", 401, String(error)) };
  }
}
