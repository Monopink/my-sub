"use client";

import type {
  AliasMapping,
  Profile,
  PullLog,
  Source,
  Template,
} from "@/modules/subscription/domain/entities";

type ApiErrorBody = {
  error?: string;
  details?: unknown;
};

export class ApiError extends Error {
  status: number;
  details: unknown;

  constructor(message: string, status: number, details: unknown = null) {
    super(message);
    this.status = status;
    this.details = details;
  }
}

async function requestJson<T>(input: string, init?: RequestInit): Promise<T> {
  const response = await fetch(input, {
    ...init,
    headers: {
      "content-type": "application/json",
      ...(init?.headers ?? {}),
    },
    cache: "no-store",
  });

  const text = await response.text();
  const body = text ? (JSON.parse(text) as unknown) : null;

  if (!response.ok) {
    const maybe = (body ?? {}) as ApiErrorBody;
    throw new ApiError(
      maybe.error ?? `request failed: ${response.status}`,
      response.status,
      maybe.details ?? body
    );
  }

  return body as T;
}

export async function listProfiles(): Promise<Profile[]> {
  const body = await requestJson<{ items: Profile[] }>("/api/admin/profiles");
  return body.items;
}

export async function getProfile(profileId: string): Promise<Profile> {
  return requestJson(`/api/admin/profiles/${encodeURIComponent(profileId)}`);
}

export async function createProfile(payload: Omit<Profile, "id" | "updatedAt">): Promise<Profile> {
  return requestJson("/api/admin/profiles", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export async function updateProfile(
  profileId: string,
  payload: Partial<Omit<Profile, "id" | "updatedAt">>
): Promise<Profile> {
  return requestJson(`/api/admin/profiles/${encodeURIComponent(profileId)}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });
}

export async function removeProfile(profileId: string): Promise<void> {
  await requestJson(`/api/admin/profiles/${encodeURIComponent(profileId)}`, {
    method: "DELETE",
  });
}

export async function listSources(): Promise<Source[]> {
  const body = await requestJson<{ items: Source[] }>("/api/admin/sources");
  return body.items;
}

export async function getSource(sourceId: string): Promise<Source> {
  return requestJson(`/api/admin/sources/${encodeURIComponent(sourceId)}`);
}

export async function createSource(payload: Omit<Source, "id" | "updatedAt">): Promise<Source> {
  return requestJson("/api/admin/sources", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export async function updateSource(
  sourceId: string,
  payload: Partial<Omit<Source, "id" | "updatedAt">>
): Promise<Source> {
  return requestJson(`/api/admin/sources/${encodeURIComponent(sourceId)}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });
}

export async function removeSource(sourceId: string): Promise<void> {
  await requestJson(`/api/admin/sources/${encodeURIComponent(sourceId)}`, {
    method: "DELETE",
  });
}

export async function listTemplates(): Promise<Template[]> {
  const body = await requestJson<{ items: Template[] }>("/api/admin/templates");
  return body.items;
}

export async function getTemplate(templateId: string): Promise<Template> {
  return requestJson(`/api/admin/templates/${encodeURIComponent(templateId)}`);
}

export async function createTemplate(
  payload: Omit<Template, "id" | "updatedAt">
): Promise<Template> {
  return requestJson("/api/admin/templates", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export async function updateTemplate(
  templateId: string,
  payload: Partial<Omit<Template, "id" | "updatedAt">>
): Promise<Template> {
  return requestJson(`/api/admin/templates/${encodeURIComponent(templateId)}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });
}

export async function removeTemplate(templateId: string): Promise<void> {
  await requestJson(`/api/admin/templates/${encodeURIComponent(templateId)}`, {
    method: "DELETE",
  });
}

export async function listAliases(): Promise<AliasMapping[]> {
  const body = await requestJson<{ items: AliasMapping[] }>("/api/admin/aliases");
  return body.items;
}

export async function createAlias(
  payload: Omit<AliasMapping, "updatedAt">
): Promise<AliasMapping> {
  return requestJson("/api/admin/aliases", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export async function updateAlias(
  alias: string,
  payload: Partial<Omit<AliasMapping, "alias" | "updatedAt">>
): Promise<AliasMapping> {
  return requestJson(`/api/admin/aliases/${encodeURIComponent(alias)}`, {
    method: "PUT",
    body: JSON.stringify(payload),
  });
}

export async function removeAlias(alias: string): Promise<void> {
  await requestJson(`/api/admin/aliases/${encodeURIComponent(alias)}`, {
    method: "DELETE",
  });
}

export type PullLogsResponse = {
  dateFrom: string;
  dateTo: string;
  limit: number;
  offset: number;
  total: number;
  hasMore: boolean;
  items: PullLog[];
};

export async function listPullLogs(params: {
  dateFrom: string;
  dateTo: string;
  alias?: string;
  status?: string;
  limit?: number;
  offset?: number;
}): Promise<PullLogsResponse> {
  const query = new URLSearchParams();
  query.set("dateFrom", params.dateFrom);
  query.set("dateTo", params.dateTo);
  if (params.alias) {
    query.set("alias", params.alias);
  }
  if (params.status) {
    query.set("status", params.status);
  }
  query.set("limit", `${params.limit ?? 50}`);
  query.set("offset", `${params.offset ?? 0}`);
  return requestJson(`/api/admin/logs/pulls?${query.toString()}`);
}

export function formatApiError(error: unknown): string {
  if (error instanceof ApiError) {
    const details =
      typeof error.details === "string"
        ? error.details
        : error.details
          ? JSON.stringify(error.details)
          : "";
    return details
      ? `${error.message} (${error.status}): ${details}`
      : `${error.message} (${error.status})`;
  }
  return String(error);
}
