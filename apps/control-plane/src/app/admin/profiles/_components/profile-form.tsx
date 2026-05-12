"use client";

import { useEffect, useMemo, type ReactNode } from "react";
import { useForm, type UseFormRegisterReturn } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import type {
  ConverterOptions,
  Profile,
  Source,
  Template,
} from "@/modules/subscription/domain/entities";
import { SUPPORTED_TARGETS } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";

const BOOL_VALUES = ["true", "false"] as const;
type BoolValue = (typeof BOOL_VALUES)[number];
type BooleanConverterOptionKey =
  | "addEmoji"
  | "removeEmoji"
  | "appendType"
  | "insert"
  | "prepend"
  | "tfo"
  | "udp"
  | "scv"
  | "tls13"
  | "list"
  | "sort"
  | "fdn"
  | "newName"
  | "strict"
  | "script"
  | "classic"
  | "expand";

const schema = z
  .object({
    name: z.string().min(1, "name is required"),
    enabled: z.boolean(),
    target: z.enum(SUPPORTED_TARGETS),
    templateId: z.string().min(1, "template is required"),
    sourceIds: z.array(z.string()),

    includeEnabled: z.boolean(),
    includeValue: z.string(),
    excludeEnabled: z.boolean(),
    excludeValue: z.string(),
    renameEnabled: z.boolean(),
    renameValue: z.string(),
    sortScriptEnabled: z.boolean(),
    sortScriptValue: z.string(),
    filterEnabled: z.boolean(),
    filterValue: z.string(),

    intervalEnabled: z.boolean(),
    intervalValue: z.string(),
    verEnabled: z.boolean(),
    verValue: z.string(),

    addEmojiEnabled: z.boolean(),
    addEmojiValue: z.enum(BOOL_VALUES),
    removeEmojiEnabled: z.boolean(),
    removeEmojiValue: z.enum(BOOL_VALUES),
    appendTypeEnabled: z.boolean(),
    appendTypeValue: z.enum(BOOL_VALUES),
    insertEnabled: z.boolean(),
    insertValue: z.enum(BOOL_VALUES),
    prependEnabled: z.boolean(),
    prependValue: z.enum(BOOL_VALUES),
    tfoEnabled: z.boolean(),
    tfoValue: z.enum(BOOL_VALUES),
    udpEnabled: z.boolean(),
    udpValue: z.enum(BOOL_VALUES),
    scvEnabled: z.boolean(),
    scvValue: z.enum(BOOL_VALUES),
    tls13Enabled: z.boolean(),
    tls13Value: z.enum(BOOL_VALUES),
    listEnabled: z.boolean(),
    listValue: z.enum(BOOL_VALUES),
    sortEnabled: z.boolean(),
    sortValue: z.enum(BOOL_VALUES),
    fdnEnabled: z.boolean(),
    fdnValue: z.enum(BOOL_VALUES),
    newNameEnabled: z.boolean(),
    newNameValue: z.enum(BOOL_VALUES),
    strictEnabled: z.boolean(),
    strictValue: z.enum(BOOL_VALUES),
    scriptEnabled: z.boolean(),
    scriptValue: z.enum(BOOL_VALUES),
    classicEnabled: z.boolean(),
    classicValue: z.enum(BOOL_VALUES),
    expandEnabled: z.boolean(),
    expandValue: z.enum(BOOL_VALUES),

    notes: z.string(),
  })
  .superRefine((value, ctx) => {
    const nonEmptyTextChecks: Array<[boolean, string, keyof typeof value]> = [
      [value.includeEnabled, value.includeValue, "includeValue"],
      [value.excludeEnabled, value.excludeValue, "excludeValue"],
      [value.renameEnabled, value.renameValue, "renameValue"],
      [value.sortScriptEnabled, value.sortScriptValue, "sortScriptValue"],
      [value.filterEnabled, value.filterValue, "filterValue"],
    ];

    for (const [enabled, raw, key] of nonEmptyTextChecks) {
      if (enabled && !raw.trim()) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          path: [key],
          message: "value is required when enabled",
        });
      }
    }

    const intChecks: Array<[boolean, string, keyof typeof value]> = [
      [value.intervalEnabled, value.intervalValue, "intervalValue"],
      [value.verEnabled, value.verValue, "verValue"],
    ];

    for (const [enabled, raw, key] of intChecks) {
      if (!enabled) {
        continue;
      }
      const normalized = raw.trim();
      const parsed = Number.parseInt(normalized, 10);
      if (!normalized || !Number.isFinite(parsed) || parsed <= 0) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          path: [key],
          message: "must be a positive integer when enabled",
        });
      }
    }
  });

export type ProfileFormValue = z.infer<typeof schema>;

function boolToValue(value: boolean): BoolValue {
  return value ? "true" : "false";
}

function parseBoolValue(value: BoolValue): boolean {
  return value === "true";
}

function parsePositiveInt(text: string): number | undefined {
  const normalized = text.trim();
  if (!normalized) {
    return undefined;
  }
  const value = Number.parseInt(normalized, 10);
  if (!Number.isFinite(value) || value <= 0) {
    return undefined;
  }
  return value;
}

function optionEnabled(value: unknown): boolean {
  if (typeof value === "boolean") {
    return true;
  }
  if (typeof value === "number") {
    return true;
  }
  if (typeof value === "string") {
    return value.trim().length > 0;
  }
  return false;
}

export function buildConverterOptionsFromForm(value: ProfileFormValue): ConverterOptions {
  const options: ConverterOptions = {};

  if (value.includeEnabled) {
    options.include = value.includeValue.trim();
  }
  if (value.excludeEnabled) {
    options.exclude = value.excludeValue.trim();
  }
  if (value.renameEnabled) {
    options.rename = value.renameValue.trim();
  }
  if (value.sortScriptEnabled) {
    options.sortScript = value.sortScriptValue.trim();
  }
  if (value.filterEnabled) {
    options.filter = value.filterValue.trim();
  }
  if (value.intervalEnabled) {
    const parsed = parsePositiveInt(value.intervalValue);
    if (typeof parsed === "number") {
      options.interval = parsed;
    }
  }
  if (value.verEnabled) {
    const parsed = parsePositiveInt(value.verValue);
    if (typeof parsed === "number") {
      options.ver = parsed;
    }
  }

  const booleanFields: Array<[enabled: boolean, raw: BoolValue, target: BooleanConverterOptionKey]> = [
    [value.addEmojiEnabled, value.addEmojiValue, "addEmoji"],
    [value.removeEmojiEnabled, value.removeEmojiValue, "removeEmoji"],
    [value.appendTypeEnabled, value.appendTypeValue, "appendType"],
    [value.insertEnabled, value.insertValue, "insert"],
    [value.prependEnabled, value.prependValue, "prepend"],
    [value.tfoEnabled, value.tfoValue, "tfo"],
    [value.udpEnabled, value.udpValue, "udp"],
    [value.scvEnabled, value.scvValue, "scv"],
    [value.tls13Enabled, value.tls13Value, "tls13"],
    [value.listEnabled, value.listValue, "list"],
    [value.sortEnabled, value.sortValue, "sort"],
    [value.fdnEnabled, value.fdnValue, "fdn"],
    [value.newNameEnabled, value.newNameValue, "newName"],
    [value.strictEnabled, value.strictValue, "strict"],
    [value.scriptEnabled, value.scriptValue, "script"],
    [value.classicEnabled, value.classicValue, "classic"],
    [value.expandEnabled, value.expandValue, "expand"],
  ];

  for (const [enabled, raw, target] of booleanFields) {
    if (enabled) {
      options[target] = parseBoolValue(raw);
    }
  }

  return options;
}

function OptionRow({
  label,
  enabledControl,
  valueControl,
  helperText,
}: {
  label: string;
  enabledControl: ReactNode;
  valueControl: ReactNode;
  helperText?: string;
}) {
  return (
    <div className="grid gap-1 md:grid-cols-[120px_180px_minmax(0,1fr)]">
      <div className="flex items-center gap-2 text-sm">{enabledControl}</div>
      <code className="text-xs text-[var(--muted-foreground)]">{label}</code>
      <div>{valueControl}</div>
      {helperText ? (
        <>
          <div />
          <div />
          <p className="text-xs text-[var(--muted-foreground)]">{helperText}</p>
        </>
      ) : null}
    </div>
  );
}

function BoolSelectField({
  id,
  registration,
}: {
  id: string;
  registration: UseFormRegisterReturn;
}) {
  return (
    <Select id={id} {...registration}>
      <option value="false">false</option>
      <option value="true">true</option>
    </Select>
  );
}

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
  const defaultTarget = initial?.target ?? "clash";
  const targetTemplates = useMemo(
    () => templates.filter((item) => item.target === defaultTarget && item.enabled),
    [templates, defaultTarget]
  );

  const form = useForm<ProfileFormValue>({
    resolver: zodResolver(schema),
    defaultValues: {
      name: initial?.name ?? "",
      enabled: initial?.enabled ?? true,
      target: defaultTarget,
      templateId: initial?.templateId ?? targetTemplates[0]?.id ?? "",
      sourceIds: initial?.sourceIds ?? [],

      includeEnabled: optionEnabled(initial?.converterOptions.include),
      includeValue: initial?.converterOptions.include ?? "",
      excludeEnabled: optionEnabled(initial?.converterOptions.exclude),
      excludeValue: initial?.converterOptions.exclude ?? "",
      renameEnabled: optionEnabled(initial?.converterOptions.rename),
      renameValue: initial?.converterOptions.rename ?? "",
      sortScriptEnabled: optionEnabled(initial?.converterOptions.sortScript),
      sortScriptValue: initial?.converterOptions.sortScript ?? "",
      filterEnabled: optionEnabled(initial?.converterOptions.filter),
      filterValue: initial?.converterOptions.filter ?? "",

      intervalEnabled: optionEnabled(initial?.converterOptions.interval),
      intervalValue:
        typeof initial?.converterOptions.interval === "number"
          ? String(initial.converterOptions.interval)
          : "",
      verEnabled: optionEnabled(initial?.converterOptions.ver),
      verValue:
        typeof initial?.converterOptions.ver === "number"
          ? String(initial.converterOptions.ver)
          : "",

      addEmojiEnabled: optionEnabled(initial?.converterOptions.addEmoji),
      addEmojiValue: boolToValue(initial?.converterOptions.addEmoji ?? false),
      removeEmojiEnabled: optionEnabled(initial?.converterOptions.removeEmoji),
      removeEmojiValue: boolToValue(initial?.converterOptions.removeEmoji ?? false),
      appendTypeEnabled: optionEnabled(initial?.converterOptions.appendType),
      appendTypeValue: boolToValue(initial?.converterOptions.appendType ?? false),
      insertEnabled: optionEnabled(initial?.converterOptions.insert),
      insertValue: boolToValue(initial?.converterOptions.insert ?? false),
      prependEnabled: optionEnabled(initial?.converterOptions.prepend),
      prependValue: boolToValue(initial?.converterOptions.prepend ?? false),
      tfoEnabled: optionEnabled(initial?.converterOptions.tfo),
      tfoValue: boolToValue(initial?.converterOptions.tfo ?? false),
      udpEnabled: optionEnabled(initial?.converterOptions.udp),
      udpValue: boolToValue(initial?.converterOptions.udp ?? false),
      scvEnabled: optionEnabled(initial?.converterOptions.scv),
      scvValue: boolToValue(initial?.converterOptions.scv ?? false),
      tls13Enabled: optionEnabled(initial?.converterOptions.tls13),
      tls13Value: boolToValue(initial?.converterOptions.tls13 ?? false),
      listEnabled: optionEnabled(initial?.converterOptions.list),
      listValue: boolToValue(initial?.converterOptions.list ?? false),
      sortEnabled: optionEnabled(initial?.converterOptions.sort),
      sortValue: boolToValue(initial?.converterOptions.sort ?? false),
      fdnEnabled: optionEnabled(initial?.converterOptions.fdn),
      fdnValue: boolToValue(initial?.converterOptions.fdn ?? false),
      newNameEnabled: optionEnabled(initial?.converterOptions.newName),
      newNameValue: boolToValue(initial?.converterOptions.newName ?? false),
      strictEnabled: optionEnabled(initial?.converterOptions.strict),
      strictValue: boolToValue(initial?.converterOptions.strict ?? false),
      scriptEnabled: optionEnabled(initial?.converterOptions.script),
      scriptValue: boolToValue(initial?.converterOptions.script ?? false),
      classicEnabled: optionEnabled(initial?.converterOptions.classic),
      classicValue: boolToValue(initial?.converterOptions.classic ?? false),
      expandEnabled: optionEnabled(initial?.converterOptions.expand),
      expandValue: boolToValue(initial?.converterOptions.expand ?? false),

      notes: initial?.notes ?? "",
    },
  });

  const selectedTarget = form.watch("target");
  const selectedTemplateId = form.watch("templateId");
  const templateOptions = useMemo(
    () => templates.filter((item) => item.target === selectedTarget && item.enabled),
    [templates, selectedTarget]
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
      className="grid gap-4"
    >
      <div>
        <Label htmlFor="name">Name</Label>
        <Input id="name" {...form.register("name")} />
      </div>

      <div className="grid gap-3 md:grid-cols-2">
        <div>
          <Label htmlFor="target">Target</Label>
          <Select id="target" {...form.register("target")}>
            {SUPPORTED_TARGETS.map((target) => (
              <option key={target} value={target}>
                {target}
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

      <div className="rounded-md border border-[var(--border)] p-3">
        <div className="mb-3 text-sm font-medium">Converter Options</div>
        <p className="mb-1 text-xs text-[var(--muted-foreground)]">
          Each parameter has a dedicated switch. Disabled means not sent to subconverter.
        </p>
        <p className="mb-3 text-xs text-[var(--muted-foreground)]">
          For standard Clash output, keep <code>new_name=true</code> and avoid <code>list=true</code>.
        </p>
        <div className="grid gap-2">
          <OptionRow
            label="include"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("includeEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input {...form.register("includeValue")} />}
          />
          <OptionRow
            label="exclude"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("excludeEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input {...form.register("excludeValue")} />}
          />
          <OptionRow
            label="rename"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("renameEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input {...form.register("renameValue")} />}
          />
          <OptionRow
            label="sort_script"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("sortScriptEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input {...form.register("sortScriptValue")} />}
          />
          <OptionRow
            label="filter"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("filterEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input {...form.register("filterValue")} />}
          />
          <OptionRow
            label="interval"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("intervalEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input inputMode="numeric" {...form.register("intervalValue")} />}
          />
          <OptionRow
            label="ver"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("verEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<Input inputMode="numeric" {...form.register("verValue")} />}
          />
          <OptionRow
            label="add_emoji"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("addEmojiEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="addEmojiValue" registration={form.register("addEmojiValue")} />}
          />
          <OptionRow
            label="remove_emoji"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("removeEmojiEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={
              <BoolSelectField id="removeEmojiValue" registration={form.register("removeEmojiValue")} />
            }
          />
          <OptionRow
            label="append_type"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("appendTypeEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={
              <BoolSelectField id="appendTypeValue" registration={form.register("appendTypeValue")} />
            }
          />
          <OptionRow
            label="insert"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("insertEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={
              <BoolSelectField id="insertValue" registration={form.register("insertValue")} />
            }
          />
          <OptionRow
            label="prepend"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("prependEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={
              <BoolSelectField id="prependValue" registration={form.register("prependValue")} />
            }
          />
          <OptionRow
            label="tfo"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("tfoEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="tfoValue" registration={form.register("tfoValue")} />}
          />
          <OptionRow
            label="udp"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("udpEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="udpValue" registration={form.register("udpValue")} />}
          />
          <OptionRow
            label="scv"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("scvEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="scvValue" registration={form.register("scvValue")} />}
          />
          <OptionRow
            label="tls13"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("tls13Enabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="tls13Value" registration={form.register("tls13Value")} />}
          />
          <OptionRow
            label="list"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("listEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="listValue" registration={form.register("listValue")} />}
            helperText="When list=true, output is node-list mode, not full Clash profile."
          />
          <OptionRow
            label="sort"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("sortEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="sortValue" registration={form.register("sortValue")} />}
          />
          <OptionRow
            label="fdn"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("fdnEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="fdnValue" registration={form.register("fdnValue")} />}
          />
          <OptionRow
            label="new_name"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("newNameEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={
              <BoolSelectField id="newNameValue" registration={form.register("newNameValue")} />
            }
            helperText="Set true for modern Clash keys: proxies / proxy-groups."
          />
          <OptionRow
            label="strict"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("strictEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="strictValue" registration={form.register("strictValue")} />}
          />
          <OptionRow
            label="script"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("scriptEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="scriptValue" registration={form.register("scriptValue")} />}
          />
          <OptionRow
            label="classic"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("classicEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={
              <BoolSelectField id="classicValue" registration={form.register("classicValue")} />
            }
          />
          <OptionRow
            label="expand"
            enabledControl={
              <label className="flex items-center gap-2">
                <Checkbox {...form.register("expandEnabled")} />
                <span>Enable</span>
              </label>
            }
            valueControl={<BoolSelectField id="expandValue" registration={form.register("expandValue")} />}
          />
        </div>
      </div>

      <label className="flex items-center gap-2 text-sm">
        <Checkbox {...form.register("enabled")} />
        <span>Profile Enabled</span>
      </label>

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
