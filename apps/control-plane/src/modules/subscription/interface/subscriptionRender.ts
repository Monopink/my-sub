import { NextResponse } from "next/server";
import { getCloudflareContext } from "@opennextjs/cloudflare";
import type {
  AliasMapping,
  Profile,
  Source,
  TargetType,
  Template,
} from "@/modules/subscription/domain/entities";
import { assertAlias } from "@/modules/subscription/domain/rules";
import { kvGetJson, kvSetJson } from "@/modules/subscription/infrastructure/kvClient";
import { KV_KEYS } from "@/modules/subscription/infrastructure/kvKeys";
import { clientIpFromHeaders } from "@/modules/subscription/interface/http";
import { getSubscriptionService } from "@/modules/subscription/interface/container";

const RESERVED_ALIASES = new Set([
  "api",
  "admin",
  "health",
  "favicon.ico",
  "robots.txt",
  "sitemap.xml",
]);

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

function notFoundResponse(): NextResponse {
  return new NextResponse("Not Found", { status: 404 });
}

type ResolvedAlias = {
  alias: AliasMapping;
  profile: Profile;
  template: Template;
  sources: Source[];
};

type SubscriptionCacheRecord = {
  schemaVersion: 1;
  signature: string;
  body: string;
  status: number;
  contentType: string;
  subscriptionUserinfo?: string;
  updatedAtMs: number;
};

type CacheHitState = "miss" | "hit" | "stale";

const SUB_CACHE_FRESH_SECONDS_DEFAULT = 300;
const SUB_CACHE_STALE_SECONDS_DEFAULT = 3600;
const SUB_CACHE_MIN_TTL_SECONDS = 60;
const SOURCE_UA_HEADER = "x-source-user-agent";

const SOURCE_UA_BY_TARGET: Partial<Record<TargetType, string>> = {
  clash: "clash.meta",
  clashr: "clash.meta",
  mixed: "clash.meta",
  singbox: "sing-box",
  surge: "Surge",
  quanx: "Quantumult X",
  quan: "Quantumult",
  loon: "Loon",
};

function sanitizeHeaderValue(value: string | undefined | null): string | undefined {
  const normalized = value?.trim();
  if (!normalized) {
    return undefined;
  }
  if (normalized.includes("\r") || normalized.includes("\n")) {
    return undefined;
  }
  return normalized;
}

function envKeyForTargetUa(target: TargetType): string {
  return `SUB_SOURCE_UA_TARGET_${target.toUpperCase().replace(/[^A-Z0-9]/g, "_")}`;
}

function resolveSourceUserAgent(target: TargetType): string {
  const byTargetEnv = sanitizeHeaderValue(process.env[envKeyForTargetUa(target)]);
  if (byTargetEnv) {
    return byTargetEnv;
  }
  const globalDefault = sanitizeHeaderValue(process.env.SUB_SOURCE_UA_DEFAULT);
  if (globalDefault) {
    return globalDefault;
  }
  const byTargetDefault = sanitizeHeaderValue(SOURCE_UA_BY_TARGET[target]);
  if (byTargetDefault) {
    return byTargetDefault;
  }
  return "clash.meta";
}

function resolveTemplateUrl(templateRefRaw: string): string {
  const templateRef = templateRefRaw.trim();
  if (!templateRef) {
    throw new Error("template.ref is required");
  }
  let parsed: URL;
  try {
    parsed = new URL(templateRef);
  } catch {
    throw new Error("template.ref must be a valid absolute URL");
  }
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error("template.ref must use http or https");
  }
  return templateRef;
}

function buildConverterUrl(
  alias: string,
  profile: Profile,
  template: Template,
  sourceUrls: string[]
): URL {
  const upstream = new URL("https://converter.internal/sub");
  upstream.searchParams.set("target", profile.target);
  upstream.searchParams.set("url", sourceUrls.join("|"));
  upstream.searchParams.set("config", resolveTemplateUrl(template.ref));

  const options = profile.converterOptions;
  const setStringOption = (key: string, value: string | undefined) => {
    const normalized = value?.trim();
    if (normalized) {
      upstream.searchParams.set(key, normalized);
    }
  };
  const setBooleanOption = (key: string, value: boolean | undefined) => {
    if (typeof value === "boolean") {
      upstream.searchParams.set(key, value ? "true" : "false");
    }
  };
  const setNumberOption = (key: string, value: number | undefined) => {
    if (typeof value === "number" && Number.isFinite(value) && value > 0) {
      upstream.searchParams.set(key, String(value));
    }
  };

  setStringOption("include", options.include);
  setStringOption("exclude", options.exclude);
  setStringOption("rename", options.rename);
  setStringOption("sort_script", options.sortScript);
  setStringOption("filter", options.filter);

  setBooleanOption("add_emoji", options.addEmoji);
  setBooleanOption("remove_emoji", options.removeEmoji);
  setBooleanOption("append_type", options.appendType);
  setBooleanOption("append_info", options.appendInfo);
  setBooleanOption("insert", options.insert);
  setBooleanOption("prepend", options.prepend);
  setBooleanOption("tfo", options.tfo);
  setBooleanOption("udp", options.udp);
  setBooleanOption("scv", options.scv);
  setBooleanOption("tls13", options.tls13);
  setBooleanOption("list", options.list);
  setBooleanOption("sort", options.sort);
  setBooleanOption("fdn", options.fdn);
  setBooleanOption("new_name", options.newName);
  setBooleanOption("strict", options.strict);
  setBooleanOption("script", options.script);
  setBooleanOption("classic", options.classic);
  setBooleanOption("expand", options.expand);

  setNumberOption("interval", options.interval);
  setNumberOption("ver", options.ver);
  setNumberOption("append_info_n", options.appendInfoN);

  const filename = alias;
  upstream.searchParams.set("filename", filename);
  return upstream;
}

type ConverterService = {
  fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
};

declare global {
  interface CloudflareEnv {
    CONVERTER_SERVICE?: ConverterService;
  }
}

function isAbortError(error: unknown): boolean {
  if (!(error instanceof Error)) {
    return false;
  }
  return error.name === "AbortError" || error.message.includes("aborted");
}

async function proxyConverter(request: Request, url: URL): Promise<Response> {
  const ctx = await getCloudflareContext({ async: true });
  const service = ctx.env.CONVERTER_SERVICE;
  if (!service) {
    throw new Error("Cloudflare service binding `CONVERTER_SERVICE` is required");
  }
  const timeoutMs = Number.parseInt(process.env.SUBCONVERTER_TIMEOUT_MS ?? "45000", 10);
  const effectiveTimeoutMs = Number.isFinite(timeoutMs) && timeoutMs > 0 ? timeoutMs : 45000;
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), effectiveTimeoutMs);
  const passthroughHeaders = new Headers();
  const ua = request.headers.get("user-agent");
  if (ua) {
    passthroughHeaders.set("user-agent", ua);
  }
  const sourceUa = sanitizeHeaderValue(request.headers.get(SOURCE_UA_HEADER));
  if (sourceUa) {
    passthroughHeaders.set(SOURCE_UA_HEADER, sourceUa);
  }
  try {
    return await service.fetch(url.toString(), {
      method: "GET",
      headers: passthroughHeaders,
      signal: controller.signal,
      cache: "no-store",
    });
  } catch (error) {
    if (isAbortError(error)) {
      throw new Error(`converter request timeout after ${effectiveTimeoutMs}ms`);
    }
    throw error;
  } finally {
    clearTimeout(timer);
  }
}

function copyResponseHeaders(upstream: Response, body: string): Headers {
  const contentType = upstream.headers.get("content-type");
  const subscriptionUserinfo = upstream.headers.get("subscription-userinfo");
  return createSubscriptionHeaders(
    body,
    contentType ?? "text/plain; charset=utf-8",
    subscriptionUserinfo
  );
}

function createSubscriptionHeaders(
  body: string,
  contentType: string,
  subscriptionUserinfo?: string | null
): Headers {
  const headers = new Headers();
  headers.set("content-type", contentType);
  if (subscriptionUserinfo && subscriptionUserinfo.trim()) {
    headers.set("subscription-userinfo", subscriptionUserinfo);
  }
  headers.set("content-length", new TextEncoder().encode(body).length.toString());
  return headers;
}

function parsePositiveInt(raw: string | undefined, fallback: number): number {
  const parsed = Number.parseInt(raw ?? "", 10);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    return fallback;
  }
  return parsed;
}

function getCachePolicy(): {
  freshMs: number;
  staleMs: number;
  ttlSeconds: number;
} {
  const freshSeconds = parsePositiveInt(
    process.env.SUB_CACHE_FRESH_SECONDS,
    SUB_CACHE_FRESH_SECONDS_DEFAULT
  );
  const staleSeconds = parsePositiveInt(
    process.env.SUB_CACHE_STALE_SECONDS,
    SUB_CACHE_STALE_SECONDS_DEFAULT
  );
  const normalizedStaleSeconds = Math.max(staleSeconds, freshSeconds);
  const ttlSeconds = Math.max(normalizedStaleSeconds + 120, SUB_CACHE_MIN_TTL_SECONDS);
  return {
    freshMs: freshSeconds * 1000,
    staleMs: normalizedStaleSeconds * 1000,
    ttlSeconds,
  };
}

function buildSignature(resolved: ResolvedAlias, sourceUserAgent: string): string {
  const options = Object.entries(resolved.profile.converterOptions ?? {})
    .filter(([, value]) => {
      if (value === undefined || value === null) {
        return false;
      }
      if (typeof value === "string") {
        return value.trim().length > 0;
      }
      return true;
    })
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([key, value]) => `${key}=${String(value)}`)
    .join("&");

  const sourceRefs = resolved.sources
    .map((source) => `${source.id}|${source.updatedAt}|${source.url}`)
    .join("||");

  return [
    `alias=${resolved.alias.alias}`,
    `profile=${resolved.profile.id}`,
    `profileUpdatedAt=${resolved.profile.updatedAt}`,
    `target=${resolved.profile.target}`,
    `template=${resolved.template.id}`,
    `templateUpdatedAt=${resolved.template.updatedAt}`,
    `templateRef=${resolved.template.ref}`,
    `sourceRefs=${sourceRefs}`,
    `options=${options}`,
    `sourceUserAgent=${sourceUserAgent}`,
  ].join("\n");
}

function withCacheHeader(headers: Headers, state: CacheHitState): Headers {
  const nextHeaders = new Headers(headers);
  nextHeaders.set("x-sub-cache", state);
  return nextHeaders;
}

function shouldCacheResponse(status: number): boolean {
  return status >= 200 && status < 400;
}

async function readCache(alias: string): Promise<SubscriptionCacheRecord | null> {
  const cached = await kvGetJson<SubscriptionCacheRecord>(KV_KEYS.subCache(alias));
  if (!cached || cached.schemaVersion !== 1 || !cached.body) {
    return null;
  }
  if (typeof cached.updatedAtMs !== "number" || !Number.isFinite(cached.updatedAtMs)) {
    return null;
  }
  return cached;
}

async function writeCache(
  alias: string,
  record: SubscriptionCacheRecord,
  ttlSeconds: number
): Promise<void> {
  await kvSetJson(KV_KEYS.subCache(alias), record, ttlSeconds);
}

async function refreshCacheInBackground(
  request: Request,
  alias: string,
  signature: string,
  sourceUserAgent: string,
  upstreamUrl: URL,
  ttlSeconds: number
): Promise<void> {
  try {
    const proxyHeaders = new Headers(request.headers);
    proxyHeaders.set(SOURCE_UA_HEADER, sourceUserAgent);
    const proxyRequest = new Request(request.url, {
      method: request.method,
      headers: proxyHeaders,
    });
    const upstream = await proxyConverter(proxyRequest, upstreamUrl);
    const status = upstream.status;
    const body = await upstream.text();
    if (!shouldCacheResponse(status)) {
      return;
    }
    const record: SubscriptionCacheRecord = {
      schemaVersion: 1,
      signature,
      body,
      status,
      contentType: upstream.headers.get("content-type") ?? "text/plain; charset=utf-8",
      subscriptionUserinfo: upstream.headers.get("subscription-userinfo") ?? undefined,
      updatedAtMs: Date.now(),
    };
    await writeCache(alias, record, ttlSeconds);
  } catch (error) {
    console.error("[sub-cache] background refresh failed", alias, error);
  }
}

async function fetchAndBuildResponse(
  request: Request,
  alias: string,
  signature: string,
  sourceUserAgent: string,
  upstreamUrl: URL,
  ttlSeconds: number
): Promise<{ response: NextResponse; status: number; resultBytes: number }> {
  const proxyHeaders = new Headers(request.headers);
  proxyHeaders.set(SOURCE_UA_HEADER, sourceUserAgent);
  const proxyRequest = new Request(request.url, {
    method: request.method,
    headers: proxyHeaders,
  });
  const upstream = await proxyConverter(proxyRequest, upstreamUrl);
  const body = await upstream.text();
  const status = upstream.status;
  const resultBytes = new TextEncoder().encode(body).length;
  const headers = withCacheHeader(copyResponseHeaders(upstream, body), "miss");
  if (shouldCacheResponse(status)) {
    const record: SubscriptionCacheRecord = {
      schemaVersion: 1,
      signature,
      body,
      status,
      contentType: upstream.headers.get("content-type") ?? "text/plain; charset=utf-8",
      subscriptionUserinfo: upstream.headers.get("subscription-userinfo") ?? undefined,
      updatedAtMs: Date.now(),
    };
    await writeCache(alias, record, ttlSeconds);
  }
  return {
    response: new NextResponse(body, { status, headers }),
    status,
    resultBytes,
  };
}

export async function renderAliasSubscription(
  request: Request,
  alias: string
): Promise<NextResponse> {
  if (RESERVED_ALIASES.has(alias)) {
    return notFoundResponse();
  }
  try {
    assertAlias(alias);
  } catch (error) {
    return notFoundResponse();
  }

  const startedAt = Date.now();
  const svc = getSubscriptionService();
  const ip = clientIpFromHeaders(request.headers);
  const ua = request.headers.get("user-agent") ?? "";

  try {
    const resolved = await svc.resolveAlias(alias);
    if (!resolved) {
      await svc.appendPullLog({
        alias,
        profileId: "unknown",
        ip,
        ua,
        status: 404,
        latencyMs: Date.now() - startedAt,
        resultBytes: 0,
        error: "alias or profile not found",
      });
      return notFoundResponse();
    }

    if (resolved.sources.length === 0) {
      await svc.appendPullLog({
        alias,
        profileId: resolved.profile.id,
        ip,
        ua,
        status: 400,
        latencyMs: Date.now() - startedAt,
        resultBytes: 0,
        error: "no enabled source",
      });
      return fail("no enabled source", 400);
    }

    const upstreamUrl = buildConverterUrl(
      resolved.alias.alias,
      resolved.profile,
      resolved.template,
      resolved.sources.map((s) => s.url)
    );
    const sourceUserAgent = resolveSourceUserAgent(resolved.profile.target);
    const signature = buildSignature(resolved, sourceUserAgent);
    const cachePolicy = getCachePolicy();
    const nowMs = Date.now();
    const cache = await readCache(resolved.alias.alias);
    let status: number;
    let resultBytes: number;
    let response: NextResponse;

    if (cache && cache.signature === signature) {
      const ageMs = nowMs - cache.updatedAtMs;
      if (ageMs <= cachePolicy.freshMs) {
        const headers = withCacheHeader(
          createSubscriptionHeaders(
            cache.body,
            cache.contentType,
            cache.subscriptionUserinfo
          ),
          "hit"
        );
        status = cache.status;
        resultBytes = new TextEncoder().encode(cache.body).length;
        response = new NextResponse(cache.body, { status, headers });
      } else if (ageMs <= cachePolicy.staleMs) {
        const headers = withCacheHeader(
          createSubscriptionHeaders(
            cache.body,
            cache.contentType,
            cache.subscriptionUserinfo
          ),
          "stale"
        );
        status = cache.status;
        resultBytes = new TextEncoder().encode(cache.body).length;
        response = new NextResponse(cache.body, { status, headers });
        try {
          const context = await getCloudflareContext({ async: true });
          context.ctx.waitUntil(
            refreshCacheInBackground(
              request,
              resolved.alias.alias,
              signature,
              sourceUserAgent,
              upstreamUrl,
              cachePolicy.ttlSeconds
            )
          );
        } catch (error) {
          console.error("[sub-cache] waitUntil unavailable", error);
        }
      } else {
        const fetched = await fetchAndBuildResponse(
          request,
          resolved.alias.alias,
          signature,
          sourceUserAgent,
          upstreamUrl,
          cachePolicy.ttlSeconds
        );
        status = fetched.status;
        resultBytes = fetched.resultBytes;
        response = fetched.response;
      }
    } else {
      const fetched = await fetchAndBuildResponse(
        request,
        resolved.alias.alias,
        signature,
        sourceUserAgent,
        upstreamUrl,
        cachePolicy.ttlSeconds
      );
      status = fetched.status;
      resultBytes = fetched.resultBytes;
      response = fetched.response;
    }

    await svc.appendPullLog({
      alias: resolved.alias.alias,
      profileId: resolved.profile.id,
      ip,
      ua,
      status,
      latencyMs: Date.now() - startedAt,
      resultBytes,
      error: status >= 400 ? `upstream status ${status}` : null,
    });

    return response;
  } catch (error) {
    const timeoutError = error instanceof Error && error.message.includes("converter request timeout");
    const status = timeoutError ? 504 : 500;
    await svc.appendPullLog({
      alias,
      profileId: "unknown",
      ip,
      ua,
      status,
      latencyMs: Date.now() - startedAt,
      resultBytes: 0,
      error: String(error),
    });
    if (timeoutError) {
      return fail("upstream timeout", 504, String(error));
    }
    return fail("internal error", 500, String(error));
  }
}
