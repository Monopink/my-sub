import { SUPPORTED_TARGETS } from "@/modules/subscription/domain/entities";

const ALIAS_REGEX = /^[a-z0-9-_]+$/;

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
  if (!SUPPORTED_TARGETS.includes(target as (typeof SUPPORTED_TARGETS)[number])) {
    throw new Error(`target must be one of: ${SUPPORTED_TARGETS.join(", ")}`);
  }
}

export function nowIso(): string {
  return new Date().toISOString();
}

export function dateOnlyIso(): string {
  return new Date().toISOString().slice(0, 10);
}
