"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import type { Template } from "@/modules/subscription/domain/entities";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  formatApiError,
  getTemplate,
  updateTemplate,
} from "@/app/admin/_lib/api";
import {
  InlineError,
  InlineSuccess,
  LinkButton,
  PageTitle,
} from "@/app/admin/_components/common";
import {
  TemplateForm,
  type TemplateFormValue,
} from "@/app/admin/templates/_components/template-form";

export default function EditTemplatePage() {
  const router = useRouter();
  const params = useParams<{ templateId: string }>();
  const templateId = params.templateId ?? "";
  const [template, setTemplate] = useState<Template | null>(null);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!templateId) {
      return;
    }
    getTemplate(templateId)
      .then((value) => setTemplate(value))
      .catch((err) => setError(formatApiError(err)));
  }, [templateId]);

  async function handleSubmit(value: TemplateFormValue) {
    if (!templateId) {
      return;
    }
    try {
      setSubmitting(true);
      setError("");
      setSuccess("");
      const updated = await updateTemplate(templateId, {
        name: value.name.trim(),
        ref: value.ref.trim(),
        enabled: value.enabled,
      });
      setTemplate(updated);
      setSuccess("updated");
      router.replace(`/admin/templates/${encodeURIComponent(updated.id)}/edit`);
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="Edit Template" description="Update a template metadata entry." />
      <InlineError message={error} />
      <InlineSuccess message={success} />
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>
            {template ? (
              <>
                {template.name}
                <span className="ml-2 text-xs text-[var(--muted-foreground)]">{template.id}</span>
              </>
            ) : (
              "Loading..."
            )}
          </CardTitle>
          <LinkButton href="/admin/templates" variant="outline">
            Back
          </LinkButton>
        </CardHeader>
        <CardContent>
          {template ? (
            <TemplateForm
              mode="edit"
              initial={template}
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
