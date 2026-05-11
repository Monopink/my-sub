import { randomUUID } from "node:crypto";

export function generateEntityId(prefix: string): string {
  const token = randomUUID().replace(/-/g, "").slice(0, 12);
  return `${prefix}_${token}`;
}
