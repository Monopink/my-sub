"use client";

import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import type { Template } from "@/modules/subscription/domain/entities";
import { SUPPORTED_CLIENTS } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";

const schema = z.object({
  name: z.string().min(1, "name is required"),
  client: z.enum(SUPPORTED_CLIENTS),
  ref: z.string().min(1, "ref is required"),
  enabled: z.boolean(),
});

export type TemplateFormValue = z.infer<typeof schema>;

export function TemplateForm({
  mode,
  initial,
  submitting,
  onSubmit,
}: {
  mode: "create" | "edit";
  initial?: Template;
  submitting: boolean;
  onSubmit: (value: TemplateFormValue) => Promise<void>;
}) {
  const form = useForm<TemplateFormValue>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: initial?.name ?? "",
      client: initial?.client ?? "clash",
      ref: initial?.ref ?? "",
      enabled: initial?.enabled ?? true,
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
        <Label htmlFor="ref">Ref</Label>
        <Input
          id="ref"
          {...form.register("ref")}
          placeholder="https://raw.githubusercontent.com/Monopink/my-sub/main/base/config/main.ini"
        />
        <p className="mt-1 text-xs text-[var(--muted-foreground)]">
          Ref must be an absolute HTTP(S) URL to a plain-text template file.
        </p>
        <p className="text-xs text-[var(--muted-foreground)]">
          Clash example:{" "}
          <code>
            https://raw.githubusercontent.com/Monopink/my-sub/main/base/config/main.ini
          </code>
        </p>
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
              ? "Create Template"
              : "Update Template"}
        </Button>
      </div>
    </form>
  );
}
