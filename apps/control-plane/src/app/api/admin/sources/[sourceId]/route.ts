import { NextResponse } from "next/server";
import type { Source } from "@/modules/subscription/domain/entities";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

type RouteContext = {
  params: Promise<{ sourceId: string }>;
};

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

export async function GET(request: Request, context: RouteContext) {
  const auth = await requireAdminAccess(request);
  if (!auth.ok) {
    return auth.response;
  }
  const { sourceId } = await context.params;
  const svc = getSubscriptionService();
  const source = await svc.getSource(sourceId);
  if (!source) {
    return fail("source not found", 404);
  }
  return NextResponse.json(source);
}

export async function PUT(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { sourceId } = await context.params;
    const body = (await request.json()) as Partial<Source>;
    const svc = getSubscriptionService();
    const saved = await svc.upsertSource({
      id: sourceId,
      ...body,
    });
    return NextResponse.json(saved);
  } catch (error) {
    return fail("failed to update source", 400, String(error));
  }
}

export async function DELETE(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { sourceId } = await context.params;
    const svc = getSubscriptionService();
    const profiles = await svc.listProfiles();
    const linkedProfiles = profiles
      .filter((item) => item.sourceIds.includes(sourceId))
      .map((item) => item.id);
    if (linkedProfiles.length > 0) {
      return fail("source is referenced by profiles", 409, {
        sourceId,
        profiles: linkedProfiles,
      });
    }
    await svc.deleteSource(sourceId);
    return NextResponse.json({ success: true });
  } catch (error) {
    return fail("failed to delete source", 400, String(error));
  }
}
