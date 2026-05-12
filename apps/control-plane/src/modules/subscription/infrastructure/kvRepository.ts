import { randomUUID } from "node:crypto";
import type {
  AliasMapping,
  Profile,
  PullLog,
  Source,
  Template,
} from "@/modules/subscription/domain/entities";
import {
  assertAlias,
  assertTarget,
  assertId,
  dateOnlyIso,
  nowIso,
} from "@/modules/subscription/domain/rules";
import type { SubscriptionRepository } from "@/modules/subscription/application/ports";
import { KV_KEYS } from "@/modules/subscription/infrastructure/kvKeys";
import {
  kvDelete,
  kvGetJson,
  kvScan,
  kvSetJson,
} from "@/modules/subscription/infrastructure/kvClient";

function assertTemplateRefUrl(ref: string): string {
  const normalized = ref.trim();
  if (!normalized) {
    throw new Error("template.ref is required");
  }
  let parsed: URL;
  try {
    parsed = new URL(normalized);
  } catch {
    throw new Error("template.ref must be a valid absolute URL");
  }
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error("template.ref must use http or https");
  }
  return normalized;
}

async function scanEntities<T>(prefix: string): Promise<T[]> {
  const keys = await kvScan(`${prefix}:`);
  const items = await Promise.all(
    keys.map((key): Promise<T | null> => kvGetJson<T>(key))
  );
  const filtered: T[] = [];
  for (const item of items) {
    if (item !== null) {
      filtered.push(item as T);
    }
  }
  return filtered;
}

export class KvSubscriptionRepository implements SubscriptionRepository {
  async ensureSchemaVersion(): Promise<void> {
    const current = await kvGetJson<{ schema_version: string }>(
      KV_KEYS.schemaVersion
    );
    if (!current) {
      await kvSetJson(KV_KEYS.schemaVersion, {
        schema_version: "1.0.0",
        updated_at: nowIso(),
      });
    }
  }

  async listProfiles(): Promise<Profile[]> {
    const profiles = await scanEntities<Profile>("profile");
    return profiles.sort((a, b) => a.id.localeCompare(b.id));
  }

  async getProfile(id: string): Promise<Profile | null> {
    return kvGetJson<Profile>(KV_KEYS.profile(id));
  }

  async upsertProfile(input: Partial<Profile> & { id: string }): Promise<Profile> {
    assertId(input.id, "profile.id");
    const current = await this.getProfile(input.id);
    const resolvedTarget = input.target ?? current?.target;
    if (!resolvedTarget) {
      throw new Error("profile.target is required");
    }
    assertTarget(resolvedTarget);
    const resolvedTemplateId = input.templateId ?? current?.templateId;
    if (!resolvedTemplateId) {
      throw new Error("profile.templateId is required");
    }
    assertId(resolvedTemplateId, "profile.templateId");
    const merged: Profile = {
      id: input.id,
      name: input.name ?? current?.name ?? input.id,
      enabled: input.enabled ?? current?.enabled ?? true,
      target: resolvedTarget,
      templateId: resolvedTemplateId,
      sourceIds: input.sourceIds ?? current?.sourceIds ?? [],
      converterOptions: input.converterOptions ?? current?.converterOptions ?? {},
      notes: input.notes ?? current?.notes ?? "",
      updatedAt: nowIso(),
    };
    await kvSetJson(KV_KEYS.profile(merged.id), merged);
    return merged;
  }

  async deleteProfile(id: string): Promise<void> {
    assertId(id, "profile.id");
    await kvDelete(KV_KEYS.profile(id));
  }

  async listSources(): Promise<Source[]> {
    const sources = await scanEntities<Source>("source");
    return sources.sort((a, b) => a.id.localeCompare(b.id));
  }

  async getSource(id: string): Promise<Source | null> {
    return kvGetJson<Source>(KV_KEYS.source(id));
  }

  async upsertSource(input: Partial<Source> & { id: string }): Promise<Source> {
    assertId(input.id, "source.id");
    const current = await this.getSource(input.id);
    const merged: Source = {
      id: input.id,
      name: input.name ?? current?.name ?? input.id,
      enabled: input.enabled ?? current?.enabled ?? true,
      url: input.url ?? current?.url ?? "",
      tags: input.tags ?? current?.tags ?? [],
      updatedAt: nowIso(),
    };
    await kvSetJson(KV_KEYS.source(merged.id), merged);
    return merged;
  }

  async deleteSource(id: string): Promise<void> {
    assertId(id, "source.id");
    await kvDelete(KV_KEYS.source(id));
  }

  async listTemplates(): Promise<Template[]> {
    const templates = await scanEntities<Template>("template");
    return templates.sort((a, b) => a.id.localeCompare(b.id));
  }

  async getTemplate(id: string): Promise<Template | null> {
    return kvGetJson<Template>(KV_KEYS.template(id));
  }

  async upsertTemplate(input: Partial<Template> & { id: string }): Promise<Template> {
    assertId(input.id, "template.id");
    const current = await this.getTemplate(input.id);
    const resolvedTarget = input.target ?? current?.target;
    if (!resolvedTarget) {
      throw new Error("template.target is required");
    }
    assertTarget(resolvedTarget);
    const resolvedRef = assertTemplateRefUrl(input.ref ?? current?.ref ?? "");
    const merged: Template = {
      id: input.id,
      name: input.name ?? current?.name ?? input.id,
      enabled: input.enabled ?? current?.enabled ?? true,
      target: resolvedTarget,
      ref: resolvedRef,
      updatedAt: nowIso(),
    };
    await kvSetJson(KV_KEYS.template(merged.id), merged);
    return merged;
  }

  async deleteTemplate(id: string): Promise<void> {
    assertId(id, "template.id");
    await kvDelete(KV_KEYS.template(id));
  }

  async listAliases(): Promise<AliasMapping[]> {
    const aliases = await scanEntities<AliasMapping>("alias");
    return aliases.sort((a, b) => a.alias.localeCompare(b.alias));
  }

  async getAlias(alias: string): Promise<AliasMapping | null> {
    assertAlias(alias);
    return kvGetJson<AliasMapping>(KV_KEYS.alias(alias));
  }

  async upsertAlias(
    input: Omit<AliasMapping, "updatedAt"> & { updatedAt?: string }
  ): Promise<AliasMapping> {
    assertAlias(input.alias);
    assertId(input.profileId, "profileId");
    const merged: AliasMapping = {
      ...input,
      updatedAt: nowIso(),
    };
    await kvSetJson(KV_KEYS.alias(merged.alias), merged);
    return merged;
  }

  async deleteAlias(alias: string): Promise<void> {
    assertAlias(alias);
    await kvDelete(KV_KEYS.alias(alias));
  }

  async resolveAlias(
    alias: string
  ): Promise<{
    alias: AliasMapping;
    profile: Profile;
    template: Template;
    sources: Source[];
  } | null> {
    const mapping = await this.getAlias(alias);
    if (!mapping) {
      return null;
    }
    const profile = await this.getProfile(mapping.profileId);
    if (!profile || !profile.enabled) {
      return null;
    }
    const template = await this.getTemplate(profile.templateId);
    if (!template || !template.enabled || template.target !== profile.target) {
      return null;
    }
    const sources = await Promise.all(profile.sourceIds.map((id) => this.getSource(id)));
    const enabledSources = sources.filter(
      (item): item is Source => Boolean(item && item.enabled)
    );
    return { alias: mapping, profile, template, sources: enabledSources };
  }

  async appendPullLog(log: Omit<PullLog, "ts"> & { ts?: string }): Promise<string> {
    const date = dateOnlyIso();
    const id = randomUUID();
    const key = KV_KEYS.pullLog(date, id);
    const normalized: PullLog = {
      ...log,
      ts: log.ts ?? nowIso(),
    };
    const days = Number.parseInt(process.env.PULL_LOG_RETENTION_DAYS ?? "30", 10);
    const ttl = Number.isFinite(days) && days > 0 ? days * 24 * 60 * 60 : 30 * 24 * 60 * 60;
    await kvSetJson(key, normalized, ttl);
    return key;
  }

  async listPullLogsByDate(date: string): Promise<PullLog[]> {
    const keys = await kvScan(`log:pull:${date}:`);
    const logs = await Promise.all(keys.map((key) => kvGetJson<PullLog>(key)));
    return logs
      .filter((item): item is PullLog => Boolean(item))
      .sort((a, b) => b.ts.localeCompare(a.ts));
  }
}
