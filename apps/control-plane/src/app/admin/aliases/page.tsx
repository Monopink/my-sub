"use client";

import { useEffect, useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import type { AliasMapping, Profile } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { TD, TH, TBody, THead, TR, Table } from "@/components/ui/table";
import {
  createAlias,
  formatApiError,
  listAliases,
  listProfiles,
  removeAlias,
  updateAlias,
} from "@/app/admin/_lib/api";
import { Modal } from "@/app/admin/_components/modal";
import {
  InlineError,
  InlineSuccess,
  PageTitle,
} from "@/app/admin/_components/common";

const schema = z.object({
  alias: z.string().regex(/^[a-z0-9-_]+$/, "alias must match [a-z0-9-_]"),
  profileId: z.string().min(1, "profile is required"),
  description: z.string(),
});

type FormValue = z.infer<typeof schema>;

const EMPTY_FORM: FormValue = {
  alias: "",
  profileId: "",
  description: "",
};

export default function AliasesPage() {
  const [items, setItems] = useState<AliasMapping[]>([]);
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [loading, setLoading] = useState(false);
  const [deletingAlias, setDeletingAlias] = useState("");
  const [open, setOpen] = useState(false);
  const [editingAlias, setEditingAlias] = useState<string | null>(null);

  const form = useForm<FormValue>({
    resolver: zodResolver(schema),
    defaultValues: EMPTY_FORM,
  });

  const profileOptions = useMemo(
    () => profiles.map((item) => ({ id: item.id, label: item.name })),
    [profiles]
  );

  async function refresh() {
    try {
      setLoading(true);
      setError("");
      const [aliases, allProfiles] = await Promise.all([listAliases(), listProfiles()]);
      setItems(aliases);
      setProfiles(allProfiles);
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void refresh();
  }, []);

  function openCreate() {
    form.reset({
      ...EMPTY_FORM,
      profileId: profiles[0]?.id ?? "",
    });
    setEditingAlias(null);
    setOpen(true);
    setSuccess("");
    setError("");
  }

  function openEdit(item: AliasMapping) {
    form.reset({
      alias: item.alias,
      profileId: item.profileId,
      description: item.description ?? "",
    });
    setEditingAlias(item.alias);
    setOpen(true);
    setSuccess("");
    setError("");
  }

  async function onSubmit(value: FormValue) {
    try {
      setError("");
      setSuccess("");
      if (editingAlias) {
        await updateAlias(editingAlias, {
          profileId: value.profileId.trim(),
          description: value.description.trim(),
        });
      } else {
        await createAlias({
          alias: value.alias.trim(),
          profileId: value.profileId.trim(),
          description: value.description.trim(),
        });
      }
      setOpen(false);
      setSuccess(editingAlias ? "updated" : "created");
      await refresh();
    } catch (err) {
      setError(formatApiError(err));
    }
  }

  async function onDelete(alias: string) {
    if (!window.confirm(`Delete alias ${alias}?`)) {
      return;
    }
    try {
      setDeletingAlias(alias);
      setError("");
      setSuccess("");
      await removeAlias(alias);
      setSuccess("deleted");
      await refresh();
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setDeletingAlias("");
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="Aliases" description="Manage public alias mappings." />
      <InlineError message={error} />
      <InlineSuccess message={success} />

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Alias List</CardTitle>
          <Button
            type="button"
            onClick={openCreate}
            disabled={profiles.length === 0}
            className="text-white"
          >
            New Alias
          </Button>
        </CardHeader>
        <CardContent>
          {profiles.length === 0 ? (
            <p className="mb-3 text-sm text-[var(--muted-foreground)]">
              Create at least one profile before creating aliases.
            </p>
          ) : null}
          {loading ? <p className="text-sm">Loading...</p> : null}
          <div className="overflow-x-auto">
            <Table>
              <THead>
                <TR>
                  <TH>Alias</TH>
                  <TH>Profile</TH>
                  <TH>Description</TH>
                  <TH>Updated At</TH>
                  <TH>Actions</TH>
                </TR>
              </THead>
              <TBody>
                {items.map((item) => (
                  <TR key={item.alias}>
                    <TD>{item.alias}</TD>
                    <TD>{item.profileId}</TD>
                    <TD>{item.description ?? ""}</TD>
                    <TD>{new Date(item.updatedAt).toLocaleString()}</TD>
                    <TD>
                      <div className="flex gap-2">
                        <Button variant="outline" onClick={() => openEdit(item)}>
                          Edit
                        </Button>
                        <Button
                          variant="destructive"
                          onClick={() => onDelete(item.alias)}
                          disabled={deletingAlias === item.alias}
                        >
                          Delete
                        </Button>
                      </div>
                    </TD>
                  </TR>
                ))}
              </TBody>
            </Table>
          </div>
        </CardContent>
      </Card>

      <Modal
        open={open}
        title={editingAlias ? `Edit Alias: ${editingAlias}` : "Create Alias"}
        onClose={() => setOpen(false)}
      >
        <form onSubmit={form.handleSubmit(onSubmit)} className="grid gap-3">
          <div>
            <Label htmlFor="alias">Alias</Label>
            <Input
              id="alias"
              {...form.register("alias")}
              disabled={Boolean(editingAlias)}
            />
          </div>
          <div>
            <Label htmlFor="profileId">Profile</Label>
            <Select id="profileId" {...form.register("profileId")}>
              {profileOptions.map((profile) => (
                <option key={profile.id} value={profile.id}>
                  {profile.label} ({profile.id})
                </option>
              ))}
            </Select>
          </div>
          <div>
            <Label htmlFor="description">Description</Label>
            <Input id="description" {...form.register("description")} />
          </div>
          <div className="flex justify-end gap-2">
            <Button type="button" variant="outline" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button type="submit">{editingAlias ? "Update" : "Create"}</Button>
          </div>
        </form>
      </Modal>
    </div>
  );
}
