"use client";

import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import type { Source } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

const schema = z.object({
  name: z.string().min(1, "name is required"),
  url: z.string().url("url must be valid"),
  enabled: z.boolean(),
  tagsCsv: z.string(),
});

export type SourceFormValue = z.infer<typeof schema>;

export function SourceForm({
  mode,
  initial,
  submitting,
  onSubmit,
}: {
  mode: "create" | "edit";
  initial?: Source;
  submitting: boolean;
  onSubmit: (value: SourceFormValue) => Promise<void>;
}) {
  const form = useForm<SourceFormValue>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: initial?.name ?? "",
      url: initial?.url ?? "",
      enabled: initial?.enabled ?? true,
      tagsCsv: initial ? initial.tags.join(", ") : "",
    },
  });

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
      <div>
        <Label htmlFor="url">URL</Label>
        <Input id="url" {...form.register("url")} />
      </div>
      <div>
        <Label htmlFor="tagsCsv">Tags (comma separated)</Label>
        <Input id="tagsCsv" {...form.register("tagsCsv")} />
      </div>
      <label className="flex items-center gap-2 text-sm">
        <Checkbox {...form.register("enabled")} />
        <span>Enabled</span>
      </label>
      <div className="flex justify-end">
        <Button type="submit" disabled={submitting}>
          {submitting
            ? "Saving..."
            : mode === "create"
              ? "Create Source"
              : "Update Source"}
        </Button>
      </div>
    </form>
  );
}
