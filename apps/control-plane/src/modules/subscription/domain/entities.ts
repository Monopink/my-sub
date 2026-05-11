export const SUPPORTED_CLIENTS = [
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

export type ClientType = (typeof SUPPORTED_CLIENTS)[number];

export interface Profile {
  id: string;
  name: string;
  enabled: boolean;
  client: ClientType;
  templateId: string;
  sourceIds: string[];
  converterOptions: {
    include?: string;
    exclude?: string;
    emoji?: boolean;
  };
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
  client: ClientType;
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
