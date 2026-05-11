"use client";

import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import type { PullLog } from "@/modules/subscription/domain/entities";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select } from "@/components/ui/select";
import { TD, TH, TBody, THead, TR, Table } from "@/components/ui/table";
import {
  formatApiError,
  listPullLogs,
  type PullLogsResponse,
} from "@/app/admin/_lib/api";
import { InlineError, PageTitle } from "@/app/admin/_components/common";

function dateOnly(offsetDays = 0): string {
  const date = new Date();
  date.setUTCDate(date.getUTCDate() + offsetDays);
  return date.toISOString().slice(0, 10);
}

const schema = z.object({
  dateFrom: z.string().regex(/^\d{4}-\d{2}-\d{2}$/),
  dateTo: z.string().regex(/^\d{4}-\d{2}-\d{2}$/),
  alias: z.string(),
  status: z.string(),
  limit: z.number().int().min(1).max(200),
});

type FormValue = z.infer<typeof schema>;

const DEFAULT_FORM: FormValue = {
  dateFrom: dateOnly(-1),
  dateTo: dateOnly(0),
  alias: "",
  status: "",
  limit: 50,
};

export default function PullLogsPage() {
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<PullLogsResponse | null>(null);
  const [offset, setOffset] = useState(0);

  const form = useForm<FormValue>({
    resolver: zodResolver(schema),
    defaultValues: DEFAULT_FORM,
  });

  const activeFilter = form.watch();

  async function load(nextOffset = 0) {
    try {
      setLoading(true);
      setError("");
      const value = schema.parse(form.getValues());
      const response = await listPullLogs({
        dateFrom: value.dateFrom,
        dateTo: value.dateTo,
        alias: value.alias.trim() || undefined,
        status: value.status.trim() || undefined,
        limit: value.limit,
        offset: nextOffset,
      });
      setOffset(nextOffset);
      setResult(response);
    } catch (err) {
      setError(formatApiError(err));
    } finally {
      setLoading(false);
    }
  }

  const hasPrev = offset > 0;
  const hasNext = Boolean(result?.hasMore);
  const from = result ? offset + 1 : 0;
  const to = result ? offset + result.items.length : 0;
  const total = result?.total ?? 0;

  const subtitle = useMemo(() => {
    if (!result) {
      return "No query yet";
    }
    return `${result.dateFrom} ~ ${result.dateTo}`;
  }, [result]);

  return (
    <div className="space-y-4">
      <PageTitle
        title="Pull Logs"
        description="Query pull logs by date range, alias, and status code."
      />

      <InlineError message={error} />

      <Card>
        <CardHeader>
          <CardTitle>Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            className="grid gap-3 md:grid-cols-3"
            onSubmit={(event) => {
              event.preventDefault();
              void load(0);
            }}
          >
            <div>
              <Label htmlFor="dateFrom">Date From</Label>
              <Input id="dateFrom" type="date" {...form.register("dateFrom")} />
            </div>
            <div>
              <Label htmlFor="dateTo">Date To</Label>
              <Input id="dateTo" type="date" {...form.register("dateTo")} />
            </div>
            <div>
              <Label htmlFor="alias">Alias</Label>
              <Input id="alias" {...form.register("alias")} placeholder="optional" />
            </div>
            <div>
              <Label htmlFor="status">Status</Label>
              <Select id="status" {...form.register("status")}>
                <option value="">All</option>
                <option value="200">200</option>
                <option value="400">400</option>
                <option value="401">401</option>
                <option value="404">404</option>
                <option value="500">500</option>
                <option value="502">502</option>
                <option value="503">503</option>
              </Select>
            </div>
            <div>
              <Label htmlFor="limit">Limit</Label>
              <Select id="limit" {...form.register("limit", { valueAsNumber: true })}>
                <option value="20">20</option>
                <option value="50">50</option>
                <option value="100">100</option>
                <option value="200">200</option>
              </Select>
            </div>
            <div className="flex items-end gap-2">
              <Button type="submit" disabled={loading}>
                {loading ? "Loading..." : "Search"}
              </Button>
              <Button
                type="button"
                variant="outline"
                onClick={() => {
                  form.reset(DEFAULT_FORM);
                  setResult(null);
                  setError("");
                  setOffset(0);
                }}
              >
                Reset
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Result · {subtitle}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="flex flex-wrap items-center justify-between gap-2 text-sm">
            <span>
              Showing {from}-{to} / {total}
            </span>
            <div className="flex gap-2">
              <Button
                variant="outline"
                onClick={() => load(Math.max(0, offset - activeFilter.limit))}
                disabled={!hasPrev || loading}
              >
                Prev
              </Button>
              <Button
                variant="outline"
                onClick={() => load(offset + activeFilter.limit)}
                disabled={!hasNext || loading}
              >
                Next
              </Button>
            </div>
          </div>
          <div className="overflow-x-auto">
            <Table>
              <THead>
                <TR>
                  <TH>Time</TH>
                  <TH>Alias</TH>
                  <TH>Profile</TH>
                  <TH>Status</TH>
                  <TH>Latency</TH>
                  <TH>Bytes</TH>
                  <TH>IP</TH>
                  <TH>UA</TH>
                  <TH>Error</TH>
                </TR>
              </THead>
              <TBody>
                {(result?.items ?? []).map((item: PullLog, index) => (
                  <TR key={`${item.ts}-${item.alias}-${index}`}>
                    <TD>{new Date(item.ts).toLocaleString()}</TD>
                    <TD>{item.alias}</TD>
                    <TD>{item.profileId}</TD>
                    <TD>{item.status}</TD>
                    <TD>{item.latencyMs} ms</TD>
                    <TD>{item.resultBytes}</TD>
                    <TD>{item.ip}</TD>
                    <TD className="max-w-xs break-all">{item.ua}</TD>
                    <TD className="max-w-xs break-all">{item.error ?? ""}</TD>
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
