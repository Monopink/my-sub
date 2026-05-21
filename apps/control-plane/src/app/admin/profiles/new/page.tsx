"use client";

import { useEffect, useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import type { Profile, Source, Template } from "@/modules/subscription/domain/entities";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  createProfile,
  formatApiError,
  getProfile,
  listProfiles,
  listSources,
  listTemplates,
} from "@/app/admin/_lib/api";
import { InlineError, LinkButton, PageTitle } from "@/app/admin/_components/common";
import {
  buildOrderedSourceIdsFromForm,
  buildConverterOptionsFromForm,
  ProfileForm,
  type ProfileFormValue,
} from "@/app/admin/profiles/_components/profile-form";

function buildCopyName(baseName: string, existingNames: string[]): string {
  const used = new Set(existingNames);
  const firstCandidate = `${baseName}_copy`;
  if (!used.has(firstCandidate)) {
    return firstCandidate;
  }

  let index = 2;
  while (used.has(`${baseName}_copy${index}`)) {
    index += 1;
  }
  return `${baseName}_copy${index}`;
}

export default function NewProfilePage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const fromProfileId = searchParams.get("from")?.trim() ?? "";
  const [templates, setTemplates] = useState<Template[]>([]);
  const [sources, setSources] = useState<Source[]>([]);
  const [initial, setInitial] = useState<Profile | undefined>(undefined);
  const [bootstrapping, setBootstrapping] = useState(true);
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    setBootstrapping(true);
    setError("");
    setInitial(undefined);

    Promise.all([
      listTemplates(),
      listSources(),
      listProfiles(),
      fromProfileId ? getProfile(fromProfileId) : Promise.resolve(undefined),
    ])
      .then(([allTemplates, allSources, allProfiles, baseProfile]) => {
        setTemplates(allTemplates);
        setSources(allSources);

        if (baseProfile) {
          const duplicateName = buildCopyName(
            baseProfile.name,
            allProfiles.map((item) => item.name)
          );
          setInitial({
            ...baseProfile,
            name: duplicateName,
          });
        }
      })
      .catch((err) => setError(formatApiError(err)))
      .finally(() => setBootstrapping(false));
  }, [fromProfileId]);

  async function handleSubmit(value: ProfileFormValue) {
    try {
      setSubmitting(true);
      setError("");
      await createProfile({
        name: value.name.trim(),
        enabled: value.enabled,
        target: value.target,
        templateId: value.templateId,
        sourceIds: buildOrderedSourceIdsFromForm(value),
        converterOptions: buildConverterOptionsFromForm(value),
        notes: value.notes.trim(),
      });
      router.replace("/admin/profiles");
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle
        title={fromProfileId ? "Duplicate Profile" : "New Profile"}
        description={
          fromProfileId
            ? "Create a new profile from an existing profile."
            : "Create a new profile configuration."
        }
      />
      <InlineError message={error} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>{fromProfileId ? "Duplicate Profile" : "Create Profile"}</CardTitle>
          <LinkButton href="/admin/profiles" variant="outline">
            Back
          </LinkButton>
        </CardHeader>
        <CardContent>
          {bootstrapping ? (
            <p className="text-sm">Loading...</p>
          ) : (
            <ProfileForm
              key={fromProfileId || "create"}
              mode="create"
              initial={initial}
              templates={templates}
              sources={sources}
              submitting={submitting}
              onSubmit={handleSubmit}
            />
          )}
        </CardContent>
      </Card>
    </div>
  );
}
