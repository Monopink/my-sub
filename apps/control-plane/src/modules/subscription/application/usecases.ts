import type {
  AliasMapping,
  Profile,
  PullLog,
  Source,
  Template,
} from "@/modules/subscription/domain/entities";
import { assertAlias, assertTarget } from "@/modules/subscription/domain/rules";
import type { SubscriptionRepository } from "@/modules/subscription/application/ports";

export class SubscriptionService {
  constructor(private readonly repository: SubscriptionRepository) {}

  async health(): Promise<{ ok: true; schema: string }> {
    await this.repository.ensureSchemaVersion();
    return { ok: true, schema: "1.0.0" };
  }

  listProfiles(): Promise<Profile[]> {
    return this.repository.listProfiles();
  }

  getProfile(id: string): Promise<Profile | null> {
    return this.repository.getProfile(id);
  }

  upsertProfile(input: Partial<Profile> & { id: string }): Promise<Profile> {
    return this.validateAndUpsertProfile(input);
  }

  deleteProfile(id: string): Promise<void> {
    return this.repository.deleteProfile(id);
  }

  listSources(): Promise<Source[]> {
    return this.repository.listSources();
  }

  getSource(id: string): Promise<Source | null> {
    return this.repository.getSource(id);
  }

  upsertSource(input: Partial<Source> & { id: string }): Promise<Source> {
    const normalizedTags = input.tags
      ? Array.from(new Set(input.tags.map((tag) => tag.trim()).filter(Boolean)))
      : undefined;
    return this.repository.upsertSource({
      ...input,
      tags: normalizedTags,
    });
  }

  deleteSource(id: string): Promise<void> {
    return this.repository.deleteSource(id);
  }

  listTemplates(): Promise<Template[]> {
    return this.repository.listTemplates();
  }

  getTemplate(id: string): Promise<Template | null> {
    return this.repository.getTemplate(id);
  }

  upsertTemplate(input: Partial<Template> & { id: string }): Promise<Template> {
    return this.repository.upsertTemplate(input);
  }

  deleteTemplate(id: string): Promise<void> {
    return this.repository.deleteTemplate(id);
  }

  listAliases(): Promise<AliasMapping[]> {
    return this.repository.listAliases();
  }

  getAlias(alias: string): Promise<AliasMapping | null> {
    return this.repository.getAlias(alias);
  }

  upsertAlias(
    input: Omit<AliasMapping, "updatedAt"> & { updatedAt?: string }
  ): Promise<AliasMapping> {
    return this.validateAndUpsertAlias(input);
  }

  private async validateAndUpsertAlias(
    input: Omit<AliasMapping, "updatedAt"> & { updatedAt?: string }
  ): Promise<AliasMapping> {
    assertAlias(input.alias);
    const profile = await this.repository.getProfile(input.profileId);
    if (!profile) {
      throw new Error(`profile not found: ${input.profileId}`);
    }
    return this.repository.upsertAlias(input);
  }

  private async validateAndUpsertProfile(
    input: Partial<Profile> & { id: string }
  ): Promise<Profile> {
    if (input.target) {
      assertTarget(input.target);
    }

    const current = await this.repository.getProfile(input.id);
    const resolvedTarget = input.target ?? current?.target;
    if (!resolvedTarget) {
      throw new Error("profile.target is required");
    }

    const resolvedTemplateId = input.templateId ?? current?.templateId;
    if (!resolvedTemplateId) {
      throw new Error("profile.templateId is required");
    }
    const template = await this.repository.getTemplate(resolvedTemplateId);
    if (!template) {
      throw new Error(`template not found: ${resolvedTemplateId}`);
    }

    if (input.sourceIds) {
      const normalizedSourceIds = Array.from(
        new Set(input.sourceIds.map((id) => id.trim()).filter(Boolean))
      );
      const missing: string[] = [];
      for (const sourceId of normalizedSourceIds) {
        const source = await this.repository.getSource(sourceId);
        if (!source) {
          missing.push(sourceId);
        }
      }
      if (missing.length > 0) {
        throw new Error(`source not found: ${missing.join(", ")}`);
      }
      return this.repository.upsertProfile({
        ...input,
        sourceIds: normalizedSourceIds,
        templateId: resolvedTemplateId,
      });
    }
    return this.repository.upsertProfile({
      ...input,
      templateId: resolvedTemplateId,
    });
  }

  deleteAlias(alias: string): Promise<void> {
    assertAlias(alias);
    return this.repository.deleteAlias(alias);
  }

  resolveAlias(
    alias: string
  ): Promise<{
    alias: AliasMapping;
    profile: Profile;
    template: Template;
    sources: Source[];
  } | null> {
    assertAlias(alias);
    return this.repository.resolveAlias(alias);
  }

  appendPullLog(log: Omit<PullLog, "ts"> & { ts?: string }): Promise<string> {
    return this.repository.appendPullLog(log);
  }

  listPullLogsByDate(date: string): Promise<PullLog[]> {
    return this.repository.listPullLogsByDate(date);
  }
}
