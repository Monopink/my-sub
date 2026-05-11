"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import type { Source } from "@/modules/subscription/domain/entities";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  formatApiError,
  getSource,
  updateSource,
} from "@/app/admin/_lib/api";
import { InlineError, InlineSuccess, PageTitle } from "@/app/admin/_components/common";
import {
  SourceForm,
  type SourceFormValue,
} from "@/app/admin/sources/_components/source-form";

export default function EditSourcePage() {
  const router = useRouter();
  const params = useParams<{ sourceId: string }>();
  const sourceId = params.sourceId ?? "";
  const [source, setSource] = useState<Source | null>(null);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!sourceId) {
      return;
    }
    getSource(sourceId)
      .then((value) => setSource(value))
      .catch((err) => setError(formatApiError(err)));
  }, [sourceId]);

  async function handleSubmit(value: SourceFormValue) {
    if (!sourceId) {
      return;
    }
    try {
      setSubmitting(true);
      setError("");
      setSuccess("");
      const updated = await updateSource(sourceId, {
        name: value.name.trim(),
        url: value.url.trim(),
        enabled: value.enabled,
        tags: value.tagsCsv
          .split(",")
          .map((item) => item.trim())
          .filter(Boolean),
      });
      setSource(updated);
      setSuccess("updated");
      router.replace(`/admin/sources/${encodeURIComponent(updated.id)}/edit`);
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="Edit Source" description="Update a source entry." />
      <InlineError message={error} />
      <InlineSuccess message={success} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>
            {source ? (
              <>
                {source.name}
                <span className="ml-2 text-xs text-[var(--muted-foreground)]">{source.id}</span>
              </>
            ) : (
              "Loading..."
            )}
          </CardTitle>
          <Link
            href="/admin/sources"
            className="inline-flex h-9 items-center justify-center rounded-md border border-[var(--border)] bg-[var(--card)] px-3 text-sm font-medium text-[var(--foreground)] transition hover:bg-[var(--muted)]"
          >
            Back
          </Link>
        </CardHeader>
        <CardContent>
          {source ? (
            <SourceForm
              mode="edit"
              initial={source}
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
