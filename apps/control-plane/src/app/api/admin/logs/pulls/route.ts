import { NextResponse } from "next/server";
import { dateOnlyIso } from "@/modules/subscription/domain/rules";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

const DAY_MS = 24 * 60 * 60 * 1000;
const MAX_DATE_RANGE_DAYS = 31;

function parseDateOnly(input: string | null | undefined): Date | null {
  if (!input) {
    return null;
  }
  if (!/^\d{4}-\d{2}-\d{2}$/.test(input)) {
    return null;
  }
  const parsed = new Date(`${input}T00:00:00.000Z`);
  if (Number.isNaN(parsed.getTime())) {
    return null;
  }
  return parsed;
}

function toDateOnlyIso(date: Date): string {
  return date.toISOString().slice(0, 10);
}

function datesInRange(from: Date, to: Date): string[] {
  const result: string[] = [];
  for (let t = from.getTime(); t <= to.getTime(); t += DAY_MS) {
    result.push(toDateOnlyIso(new Date(t)));
  }
  return result;
}

function daySpanInclusive(from: Date, to: Date): number {
  return Math.floor((to.getTime() - from.getTime()) / DAY_MS) + 1;
}

function parsePositiveInt(
  value: string | null,
  fallback: number,
  max: number
): number {
  if (!value) {
    return fallback;
  }
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 0) {
    return fallback;
  }
  return Math.min(parsed, max);
}

export async function GET(request: Request) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }

    const url = new URL(request.url);
    const dateFromRaw = url.searchParams.get("dateFrom");
    const dateToRaw = url.searchParams.get("dateTo");
    const alias = url.searchParams.get("alias");
    const statusRaw = url.searchParams.get("status");
    const limit = parsePositiveInt(url.searchParams.get("limit"), 50, 200);
    const offset = parsePositiveInt(url.searchParams.get("offset"), 0, 100000);

    const defaultDate = dateOnlyIso();
    const rangeFrom = parseDateOnly(dateFromRaw ?? defaultDate);
    const rangeTo = parseDateOnly(dateToRaw ?? defaultDate);
    if (!rangeFrom || !rangeTo) {
      return fail("invalid date range", 400, "dateFrom/dateTo must be YYYY-MM-DD");
    }
    if (rangeFrom.getTime() > rangeTo.getTime()) {
      return fail("invalid date range", 400, "dateFrom cannot be after dateTo");
    }
    if (daySpanInclusive(rangeFrom, rangeTo) > MAX_DATE_RANGE_DAYS) {
      return fail(
        "invalid date range",
        400,
        `date range cannot exceed ${MAX_DATE_RANGE_DAYS} days`
      );
    }

    let statusFilter: number | null = null;
    if (statusRaw) {
      const parsed = Number.parseInt(statusRaw, 10);
      if (!Number.isFinite(parsed) || parsed < 100 || parsed > 599) {
        return fail("invalid status", 400, "status must be an HTTP status code");
      }
      statusFilter = parsed;
    }

    const svc = getSubscriptionService();

    const dates = datesInRange(rangeFrom, rangeTo);
    const chunks = await Promise.all(
      dates.map((date) => svc.listPullLogsByDate(date))
    );
    const logs = chunks
      .flat()
      .sort((a, b) => b.ts.localeCompare(a.ts));

    const filtered = logs.filter((item) => {
      if (alias && item.alias !== alias) {
        return false;
      }
      if (statusFilter !== null && item.status !== statusFilter) {
        return false;
      }
      return true;
    });

    const items = filtered.slice(offset, offset + limit);
    return NextResponse.json({
      dateFrom: toDateOnlyIso(rangeFrom),
      dateTo: toDateOnlyIso(rangeTo),
      limit,
      offset,
      total: filtered.length,
      hasMore: offset + items.length < filtered.length,
      items,
    });
  } catch (error) {
    return fail("failed to list pull logs", 500, String(error));
  }
}
