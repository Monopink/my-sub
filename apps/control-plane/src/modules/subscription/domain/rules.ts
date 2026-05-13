import { SUPPORTED_TARGETS, type TargetType } from "@/modules/subscription/domain/entities";

const ALIAS_REGEX = /^[a-z0-9-_]+$/;

const TARGET_ALIASES: Record<string, TargetType> = {
  mihomo: "clash",
  "clash-meta": "clash",
  "clash_meta": "clash",
  "clash.meta": "clash",
  "sing-box": "singbox",
  quantumult: "quan",
  quantumultx: "quanx",
  shadowrocket: "mixed",
  surfboardios: "surfboard",
};

export function normalizeTarget(target: string | null | undefined): TargetType | null {
  const normalized = target?.trim().toLowerCase();
  if (!normalized) {
    return null;
  }
  const aliased = TARGET_ALIASES[normalized] ?? normalized;
  if (SUPPORTED_TARGETS.includes(aliased as TargetType)) {
    return aliased as TargetType;
  }
  return null;
}

export function assertAlias(alias: string): void {
  if (!alias) {
    throw new Error("alias is required");
  }
  if (!ALIAS_REGEX.test(alias)) {
    throw new Error("alias must match [a-z0-9-_]");
  }
}

export function assertId(id: string, fieldName: string): void {
  if (!id || !id.trim()) {
    throw new Error(`${fieldName} is required`);
  }
}

export function assertTarget(target: string): void {
  if (!normalizeTarget(target)) {
    throw new Error(`target must be one of: ${SUPPORTED_TARGETS.join(", ")}`);
  }
}

export function nowIso(): string {
  return new Date().toISOString();
}

export function dateOnlyIso(): string {
  return new Date().toISOString().slice(0, 10);
}
