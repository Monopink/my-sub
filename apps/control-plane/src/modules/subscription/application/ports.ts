import type {
  AliasMapping,
  Profile,
  PullLog,
  Source,
  Template,
} from "@/modules/subscription/domain/entities";

export interface SubscriptionRepository {
  ensureSchemaVersion(): Promise<void>;

  listProfiles(): Promise<Profile[]>;
  getProfile(id: string): Promise<Profile | null>;
  upsertProfile(input: Partial<Profile> & { id: string }): Promise<Profile>;
  deleteProfile(id: string): Promise<void>;

  listSources(): Promise<Source[]>;
  getSource(id: string): Promise<Source | null>;
  upsertSource(input: Partial<Source> & { id: string }): Promise<Source>;
  deleteSource(id: string): Promise<void>;

  listTemplates(): Promise<Template[]>;
  getTemplate(id: string): Promise<Template | null>;
  upsertTemplate(input: Partial<Template> & { id: string }): Promise<Template>;
  deleteTemplate(id: string): Promise<void>;

  listAliases(): Promise<AliasMapping[]>;
  getAlias(alias: string): Promise<AliasMapping | null>;
  upsertAlias(
    input: Omit<AliasMapping, "updatedAt"> & { updatedAt?: string }
  ): Promise<AliasMapping>;
  deleteAlias(alias: string): Promise<void>;

  resolveAlias(
    alias: string
  ): Promise<{
    alias: AliasMapping;
    profile: Profile;
    template: Template;
    sources: Source[];
  } | null>;

  appendPullLog(log: Omit<PullLog, "ts"> & { ts?: string }): Promise<string>;
  listPullLogsByDate(date: string): Promise<PullLog[]>;
}
