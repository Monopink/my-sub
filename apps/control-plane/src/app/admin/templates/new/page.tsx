"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { createTemplate, formatApiError } from "@/app/admin/_lib/api";
import { InlineError, LinkButton, PageTitle } from "@/app/admin/_components/common";
import {
  TemplateForm,
  type TemplateFormValue,
} from "@/app/admin/templates/_components/template-form";

export default function NewTemplatePage() {
  const router = useRouter();
  const [error, setError] = useState("");
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(value: TemplateFormValue) {
    try {
      setSubmitting(true);
      setError("");
      await createTemplate({
        name: value.name.trim(),
        ref: value.ref.trim(),
        enabled: value.enabled,
      });
      router.replace("/admin/templates");
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="New Template" description="Create a template metadata entry." />
      <InlineError message={error} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Create Template</CardTitle>
          <LinkButton href="/admin/templates" variant="outline">
            Back
          </LinkButton>
        </CardHeader>
        <CardContent>
          <TemplateForm mode="create" submitting={submitting} onSubmit={handleSubmit} />
        </CardContent>
      </Card>
    </div>
  );
}
