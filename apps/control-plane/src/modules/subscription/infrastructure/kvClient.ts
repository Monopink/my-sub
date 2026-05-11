import { getCloudflareContext } from "@opennextjs/cloudflare";

type KvValue = string | null;

type KvListResult = {
  keys: Array<{ name: string }>;
  list_complete: boolean;
  cursor?: string;
};

type KvNamespaceLike = {
  get(key: string): Promise<KvValue>;
  put(key: string, value: string, options?: { expirationTtl?: number }): Promise<void>;
  delete(key: string): Promise<void>;
  list(options?: {
    prefix?: string;
    cursor?: string;
    limit?: number;
  }): Promise<KvListResult>;
};

declare global {
  interface CloudflareEnv {
    SUB_KV?: KvNamespaceLike;
  }
}

async function getBinding(): Promise<KvNamespaceLike> {
  const ctx = await getCloudflareContext({ async: true });
  const kv = ctx.env.SUB_KV;
  if (!kv) {
    throw new Error("Cloudflare KV binding `SUB_KV` is required");
  }
  return kv;
}

export async function kvGetJson<T>(key: string): Promise<T | null> {
  const kv = await getBinding();
  const raw = await kv.get(key);
  if (!raw) {
    return null;
  }
  return JSON.parse(raw) as T;
}

export async function kvSetJson(
  key: string,
  value: unknown,
  ttlSeconds?: number
): Promise<void> {
  const kv = await getBinding();
  const payload = JSON.stringify(value);
  if (ttlSeconds && ttlSeconds > 0) {
    await kv.put(key, payload, { expirationTtl: ttlSeconds });
    return;
  }
  await kv.put(key, payload);
}

export async function kvDelete(key: string): Promise<void> {
  const kv = await getBinding();
  await kv.delete(key);
}

export async function kvScan(prefix: string): Promise<string[]> {
  const kv = await getBinding();
  const keys: string[] = [];
  let cursor: string | undefined;
  do {
    const result = await kv.list({
      prefix,
      cursor,
      limit: 1000,
    });
    keys.push(...result.keys.map((item) => item.name));
    cursor = result.list_complete ? undefined : result.cursor;
  } while (cursor);
  return keys;
}

export function kvBackendName(): string {
  return "cloudflare-kv";
}
