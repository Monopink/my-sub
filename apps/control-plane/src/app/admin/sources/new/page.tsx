"use client";

import Link from "next/link";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { createSource, formatApiError } from "@/app/admin/_lib/api";
import { InlineError, PageTitle } from "@/app/admin/_components/common";
import {
  SourceForm,
  type SourceFormValue,
} from "@/app/admin/sources/_components/source-form";

export default function NewSourcePage() {
  const router = useRouter();
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(value: SourceFormValue) {
    try {
      setSubmitting(true);
      setError("");
      await createSource({
        name: value.name.trim(),
        url: value.url.trim(),
        enabled: value.enabled,
        tags: value.tagsCsv
          .split(",")
          .map((item) => item.trim())
          .filter(Boolean),
      });
      router.replace("/admin/sources");
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="New Source" description="Create a new source entry." />
      <InlineError message={error} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Create Source</CardTitle>
          <Link
            href="/admin/sources"
            className="inline-flex h-9 items-center justify-center rounded-md border border-[var(--border)] bg-[var(--card)] px-3 text-sm font-medium text-[var(--foreground)] transition hover:bg-[var(--muted)]"
          >
            Back
          </Link>
        </CardHeader>
        <CardContent>
          <SourceForm mode="create" submitting={submitting} onSubmit={handleSubmit} />
        </CardContent>
      </Card>
    </div>
  );
}
