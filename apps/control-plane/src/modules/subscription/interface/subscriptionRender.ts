import { NextResponse } from "next/server";
import { getCloudflareContext } from "@opennextjs/cloudflare";
import type { Profile, Template } from "@/modules/subscription/domain/entities";
import { assertAlias } from "@/modules/subscription/domain/rules";
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
  upstream.searchParams.set("target", profile.client);
  upstream.searchParams.set("url", sourceUrls.join("|"));
  upstream.searchParams.set("config", resolveTemplateUrl(template.ref));

  if (profile.converterOptions.include) {
    upstream.searchParams.set("include", profile.converterOptions.include);
  }
  if (profile.converterOptions.exclude) {
    upstream.searchParams.set("exclude", profile.converterOptions.exclude);
  }
  if (typeof profile.converterOptions.emoji === "boolean") {
    upstream.searchParams.set("emoji", profile.converterOptions.emoji ? "true" : "false");
  }

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

async function proxyConverter(url: URL): Promise<Response> {
  const ctx = await getCloudflareContext({ async: true });
  const service = ctx.env.CONVERTER_SERVICE;
  if (!service) {
    throw new Error("Cloudflare service binding `CONVERTER_SERVICE` is required");
  }
  const timeoutMs = Number.parseInt(process.env.SUBCONVERTER_TIMEOUT_MS ?? "45000", 10);
  const effectiveTimeoutMs = Number.isFinite(timeoutMs) && timeoutMs > 0 ? timeoutMs : 45000;
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), effectiveTimeoutMs);
  try {
    return await service.fetch(url.toString(), {
      method: "GET",
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
  const headers = new Headers();
  const contentType = upstream.headers.get("content-type");
  headers.set("content-type", contentType ?? "text/plain; charset=utf-8");
  const subscriptionUserinfo = upstream.headers.get("subscription-userinfo");
  if (subscriptionUserinfo) {
    headers.set("subscription-userinfo", subscriptionUserinfo);
  }
  headers.set("content-length", new TextEncoder().encode(body).length.toString());
  return headers;
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
    const upstream = await proxyConverter(upstreamUrl);
    const body = await upstream.text();
    const status = upstream.status;

    await svc.appendPullLog({
      alias: resolved.alias.alias,
      profileId: resolved.profile.id,
      ip,
      ua,
      status,
      latencyMs: Date.now() - startedAt,
      resultBytes: new TextEncoder().encode(body).length,
      error: status >= 400 ? `upstream status ${status}` : null,
    });

    return new NextResponse(body, {
      status,
      headers: copyResponseHeaders(upstream, body),
    });
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
