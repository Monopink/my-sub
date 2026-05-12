"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import type { Template } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { TD, TH, TBody, THead, TR, Table } from "@/components/ui/table";
import {
  formatApiError,
  listTemplates,
  removeTemplate,
} from "@/app/admin/_lib/api";
import {
  InlineError,
  InlineSuccess,
  PageTitle,
} from "@/app/admin/_components/common";

export default function TemplatesPage() {
  const [items, setItems] = useState<Template[]>([]);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [loading, setLoading] = useState(false);
  const [deletingId, setDeletingId] = useState("");

  async function refresh() {
    try {
      setLoading(true);
      setError("");
      const templates = await listTemplates();
      setItems(templates);
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    void refresh();
  }, []);

  async function onDelete(id: string) {
    if (!window.confirm(`Delete template ${id}?`)) {
      return;
    }
    try {
      setDeletingId(id);
      setError("");
      setSuccess("");
      await removeTemplate(id);
      setSuccess("deleted");
      await refresh();
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setDeletingId("");
    }
  }

  return (
    <div className="space-y-4">
      <PageTitle title="Templates" description="Manage template metadata entries." />
      <InlineError message={error} />
      <InlineSuccess message={success} />

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>Template List</CardTitle>
          <Link
            href="/admin/templates/new"
            className="inline-flex h-9 items-center justify-center rounded-md bg-[var(--primary)] px-3 text-sm font-medium text-white transition hover:brightness-110"
          >
            New Template
          </Link>
        </CardHeader>
        <CardContent>
          {loading ? <p className="text-sm">Loading...</p> : null}
          <div className="overflow-x-auto">
            <Table>
              <THead>
                <TR>
                  <TH>Name</TH>
                  <TH>Target</TH>
                  <TH>Ref</TH>
                  <TH>Enabled</TH>
                  <TH>Updated At</TH>
                  <TH>Actions</TH>
                </TR>
              </THead>
              <TBody>
                {items.map((item) => (
                  <TR key={item.id}>
                    <TD>
                      <div className="font-medium">{item.name}</div>
                      <div className="text-xs text-[var(--muted-foreground)]">{item.id}</div>
                    </TD>
                    <TD>{item.target}</TD>
                    <TD className="max-w-sm break-all">{item.ref}</TD>
                    <TD>{item.enabled ? "yes" : "no"}</TD>
                    <TD>{new Date(item.updatedAt).toLocaleString()}</TD>
                    <TD>
                      <div className="flex gap-2">
                        <Link
                          href={`/admin/templates/${encodeURIComponent(item.id)}/edit`}
                          className="inline-flex h-9 items-center justify-center rounded-md border border-[var(--border)] bg-[var(--card)] px-3 text-sm font-medium text-[var(--foreground)] transition hover:bg-[var(--muted)]"
                        >
                          Edit
                        </Link>
                        <Button
                          variant="destructive"
                          onClick={() => onDelete(item.id)}
                          disabled={deletingId === item.id}
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
    </div>
  );
}
