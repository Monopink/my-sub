export function clientIpFromHeaders(headers: Headers): string {
  const xff = headers.get("x-forwarded-for");
  if (!xff) {
    return "unknown";
  }
  return xff.split(",")[0]?.trim() || "unknown";
}

