"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import type { Profile, Source, Template } from "@/modules/subscription/domain/entities";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  formatApiError,
  getProfile,
  listSources,
  listTemplates,
  updateProfile,
} from "@/app/admin/_lib/api";
import { InlineError, InlineSuccess, PageTitle } from "@/app/admin/_components/common";
import {
  buildConverterOptionsFromForm,
  ProfileForm,
  type ProfileFormValue,
} from "@/app/admin/profiles/_components/profile-form";

export default function EditProfilePage() {
  const router = useRouter();
  const params = useParams<{ profileId: string }>();
  const profileId = params.profileId ?? "";
  const [profile, setProfile] = useState<Profile | null>(null);
  const [templates, setTemplates] = useState<Template[]>([]);
  const [sources, setSources] = useState<Source[]>([]);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!profileId) {
      return;
    }
    Promise.all([getProfile(profileId), listTemplates(), listSources()])
      .then(([profileValue, allTemplates, allSources]) => {
        setProfile(profileValue);
        setTemplates(allTemplates);
        setSources(allSources);
      })
      .catch((err) => setError(formatApiError(err)));
  }, [profileId]);

  async function handleSubmit(value: ProfileFormValue) {
    if (!profileId) {
      return;
    }
    try {
      setSubmitting(true);
      setError("");
      setSuccess("");
      const updated = await updateProfile(profileId, {
        name: value.name.trim(),
        enabled: value.enabled,
        client: value.client,
        templateId: value.templateId,
        sourceIds: value.sourceIds,
        converterOptions: buildConverterOptionsFromForm(value),
        notes: value.notes.trim(),
      });
      setProfile(updated);
      setSuccess("updated");
      router.replace(`/admin/profiles/${encodeURIComponent(updated.id)}/edit`);
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="Edit Profile" description="Update profile configuration." />
      <InlineError message={error} />
      <InlineSuccess message={success} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>
            {profile ? (
              <>
                {profile.name}
                <span className="ml-2 text-xs text-[var(--muted-foreground)]">{profile.id}</span>
              </>
            ) : (
              "Loading..."
            )}
          </CardTitle>
          <Link
            href="/admin/profiles"
            className="inline-flex h-9 items-center justify-center rounded-md border border-[var(--border)] bg-[var(--card)] px-3 text-sm font-medium text-[var(--foreground)] transition hover:bg-[var(--muted)]"
          >
            Back
          </Link>
        </CardHeader>
        <CardContent>
          {profile ? (
            <ProfileForm
              mode="edit"
              initial={profile}
              templates={templates}
              sources={sources}
              submitting={submitting}
              onSubmit={handleSubmit}
            />
          ) : (
            <p className="text-sm">Loading...</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
