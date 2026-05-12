"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import type { Source, Template } from "@/modules/subscription/domain/entities";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { createProfile, formatApiError, listSources, listTemplates } from "@/app/admin/_lib/api";
import { InlineError, PageTitle } from "@/app/admin/_components/common";
import {
  buildConverterOptionsFromForm,
  ProfileForm,
  type ProfileFormValue,
} from "@/app/admin/profiles/_components/profile-form";

export default function NewProfilePage() {
  const router = useRouter();
  const [templates, setTemplates] = useState<Template[]>([]);
  const [sources, setSources] = useState<Source[]>([]);
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    Promise.all([listTemplates(), listSources()])
      .then(([allTemplates, allSources]) => {
        setTemplates(allTemplates);
        setSources(allSources);
      })
      .catch((err) => setError(formatApiError(err)));
  }, []);

  async function handleSubmit(value: ProfileFormValue) {
    try {
      setSubmitting(true);
      setError("");
      await createProfile({
        name: value.name.trim(),
        enabled: value.enabled,
        client: value.client,
        templateId: value.templateId,
        sourceIds: value.sourceIds,
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
      <PageTitle title="New Profile" description="Create a new profile configuration." />
      <InlineError message={error} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Create Profile</CardTitle>
          <Link
            href="/admin/profiles"
            className="inline-flex h-9 items-center justify-center rounded-md border border-[var(--border)] bg-[var(--card)] px-3 text-sm font-medium text-[var(--foreground)] transition hover:bg-[var(--muted)]"
          >
            Back
          </Link>
        </CardHeader>
        <CardContent>
          <ProfileForm
            mode="create"
            templates={templates}
            sources={sources}
            submitting={submitting}
            onSubmit={handleSubmit}
          />
        </CardContent>
      </Card>
    </div>
  );
}
