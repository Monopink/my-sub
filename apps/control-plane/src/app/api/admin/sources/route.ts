import { NextResponse } from "next/server";
import type { Source } from "@/modules/subscription/domain/entities";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { generateEntityId } from "@/modules/subscription/interface/adminIds";
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
  const items = await svc.listSources();
  return NextResponse.json({ items });
}

export async function POST(request: Request) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const body = (await request.json()) as Partial<Source>;
    const svc = getSubscriptionService();
    const saved = await svc.upsertSource({
      ...body,
      id: generateEntityId("source"),
    });
    return NextResponse.json(saved, { status: 201 });
  } catch (error) {
    return fail("failed to create source", 400, String(error));
  }
}
