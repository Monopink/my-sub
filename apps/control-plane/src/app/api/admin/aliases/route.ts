import { NextResponse } from "next/server";
import type { AliasMapping } from "@/modules/subscription/domain/entities";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

export async function GET(request: Request) {
  const auth = await requireAdminAccess(request);
  if (!auth.ok) {
    return auth.response;
  }
  const svc = getSubscriptionService();
  const items = await svc.listAliases();
  return NextResponse.json({ items });
}

export async function POST(request: Request) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const body = (await request.json()) as Omit<AliasMapping, "updatedAt">;
    const svc = getSubscriptionService();
    const saved = await svc.upsertAlias(body);
    return NextResponse.json(saved, { status: 201 });
  } catch (error) {
    return fail("failed to create/update alias", 400, String(error));
  }
}
