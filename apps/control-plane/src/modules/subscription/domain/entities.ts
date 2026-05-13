export const SUPPORTED_TARGETS = [
  "auto",
  "clash",
  "clashr",
  "surge",
  "surfboard",
  "mellow",
  "sssub",
  "ss",
  "ssr",
  "v2ray",
  "trojan",
  "mixed",
  "quan",
  "quanx",
  "loon",
  "ssd",
  "singbox",
] as const;

export type TargetType = (typeof SUPPORTED_TARGETS)[number];

export interface ConverterOptions {
  include?: string;
  exclude?: string;
  rename?: string;
  addEmoji?: boolean;
  removeEmoji?: boolean;
  appendType?: boolean;
  insert?: boolean;
  prepend?: boolean;
  tfo?: boolean;
  udp?: boolean;
  scv?: boolean;
  tls13?: boolean;
  list?: boolean;
  sort?: boolean;
  sortScript?: string;
  fdn?: boolean;
  newName?: boolean;
  interval?: number;
  strict?: boolean;
  filter?: string;
  script?: boolean;
  classic?: boolean;
  expand?: boolean;
  ver?: number;
}

export interface Profile {
  id: string;
  name: string;
  enabled: boolean;
  target: TargetType;
  templateId: string;
  sourceIds: string[];
  converterOptions: ConverterOptions;
  notes?: string;
  updatedAt: string;
}

export interface Source {
  id: string;
  name: string;
  enabled: boolean;
  url: string;
  tags: string[];
  updatedAt: string;
}

export interface Template {
  id: string;
  name: string;
  enabled: boolean;
  ref: string;
  updatedAt: string;
}

export interface AliasMapping {
  alias: string;
  profileId: string;
  description?: string;
  updatedAt: string;
}

export interface PullLog {
  ts: string;
  alias: string;
  profileId: string;
  ip: string;
  ua: string;
  status: number;
  latencyMs: number;
  resultBytes: number;
  error: string | null;
}
