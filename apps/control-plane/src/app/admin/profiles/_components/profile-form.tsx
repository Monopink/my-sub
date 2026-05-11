"use client";

import { useEffect, useMemo } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import type { Profile, Source, Template } from "@/modules/subscription/domain/entities";
import { SUPPORTED_CLIENTS } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";

const schema = z.object({
  name: z.string().min(1, "name is required"),
  enabled: z.boolean(),
  client: z.enum(SUPPORTED_CLIENTS),
  templateId: z.string().min(1, "template is required"),
  sourceIds: z.array(z.string()),
  include: z.string(),
  exclude: z.string(),
  emoji: z.boolean(),
  notes: z.string(),
});

export type ProfileFormValue = z.infer<typeof schema>;

export function ProfileForm({
  mode,
  initial,
  templates,
  sources,
  submitting,
  onSubmit,
}: {
  mode: "create" | "edit";
  initial?: Profile;
  templates: Template[];
  sources: Source[];
  submitting: boolean;
  onSubmit: (value: ProfileFormValue) => Promise<void>;
}) {
  const defaultClient = initial?.client ?? "clash";
  const clientTemplates = useMemo(
    () => templates.filter((item) => item.client === defaultClient && item.enabled),
    [templates, defaultClient]
  );

  const form = useForm<ProfileFormValue>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: initial?.name ?? "",
      enabled: initial?.enabled ?? true,
      client: defaultClient,
      templateId: initial?.templateId ?? clientTemplates[0]?.id ?? "",
      sourceIds: initial?.sourceIds ?? [],
      include: initial?.converterOptions.include ?? "",
      exclude: initial?.converterOptions.exclude ?? "",
      emoji: initial?.converterOptions.emoji ?? false,
      notes: initial?.notes ?? "",
    },
  });

  const selectedClient = form.watch("client");
  const selectedTemplateId = form.watch("templateId");
  const templateOptions = useMemo(
    () => templates.filter((item) => item.client === selectedClient && item.enabled),
    [templates, selectedClient]
  );

  const selectedSourceIds = form.watch("sourceIds");

  function toggleSource(sourceId: string) {
    const hasValue = selectedSourceIds.includes(sourceId);
    const next = hasValue
      ? selectedSourceIds.filter((id) => id !== sourceId)
      : [...selectedSourceIds, sourceId];
    form.setValue("sourceIds", next, { shouldValidate: true });
  }

  useEffect(() => {
    if (templateOptions.length === 0) {
      return;
    }
    const exists = templateOptions.some((item) => item.id === selectedTemplateId);
    if (!exists) {
      form.setValue("templateId", templateOptions[0].id, { shouldValidate: true });
    }
  }, [form, selectedTemplateId, templateOptions]);

  return (
    <form
      onSubmit={form.handleSubmit(async (value) => {
        await onSubmit(value);
      })}
      className="grid gap-3"
    >
      <div>
        <Label htmlFor="name">Name</Label>
        <Input id="name" {...form.register("name")} />
      </div>
      <div className="grid gap-3 md:grid-cols-2">
        <div>
          <Label htmlFor="client">Client</Label>
          <Select id="client" {...form.register("client")}>
            {SUPPORTED_CLIENTS.map((client) => (
              <option key={client} value={client}>
                {client}
              </option>
            ))}
          </Select>
        </div>
        <div>
          <Label htmlFor="templateId">Template</Label>
          <Select id="templateId" {...form.register("templateId")}>
            {templateOptions.map((item) => (
              <option key={item.id} value={item.id}>
                {item.name} ({item.id})
              </option>
            ))}
          </Select>
        </div>
      </div>
      <div>
        <Label>Sources</Label>
        <div className="mt-2 grid gap-2 rounded-md border border-[var(--border)] p-3 md:grid-cols-2">
          {sources.map((source) => (
            <label key={source.id} className="flex items-center gap-2 text-sm">
              <Checkbox
                checked={selectedSourceIds.includes(source.id)}
                onChange={() => toggleSource(source.id)}
              />
              <span>{source.name}</span>
              <span className="text-xs text-[var(--muted-foreground)]">{source.id}</span>
            </label>
          ))}
        </div>
      </div>
      <div className="grid gap-3 md:grid-cols-2">
        <div>
          <Label htmlFor="include">Converter Include</Label>
          <Input id="include" {...form.register("include")} />
        </div>
        <div>
          <Label htmlFor="exclude">Converter Exclude</Label>
          <Input id="exclude" {...form.register("exclude")} />
        </div>
      </div>
      <div className="flex items-center gap-6">
        <label className="flex items-center gap-2 text-sm">
          <Checkbox {...form.register("enabled")} />
          <span>Enabled</span>
        </label>
        <label className="flex items-center gap-2 text-sm">
          <Checkbox {...form.register("emoji")} />
          <span>Emoji</span>
        </label>
      </div>
      <div>
        <Label htmlFor="notes">Notes</Label>
        <Textarea id="notes" {...form.register("notes")} />
      </div>
      <div className="flex justify-end gap-2">
        <Button type="submit" disabled={submitting}>
          {submitting
            ? "Saving..."
            : mode === "create"
              ? "Create Profile"
              : "Update Profile"}
        </Button>
      </div>
    </form>
  );
}
