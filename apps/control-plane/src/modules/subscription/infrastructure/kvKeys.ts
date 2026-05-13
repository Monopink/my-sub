export const KV_KEYS = {
  schemaVersion: "meta:version",
  profile: (id: string) => `profile:${id}`,
  source: (id: string) => `source:${id}`,
  template: (id: string) => `template:${id}`,
  alias: (alias: string) => `alias:${alias}`,
  subCache: (alias: string) => `cache:sub:${alias}`,
  pullLog: (date: string, id: string) => `log:pull:${date}:${id}`,
} as const;
