import { NextResponse } from "next/server";
import type { AliasMapping } from "@/modules/subscription/domain/entities";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

type RouteContext = {
  params: Promise<{ alias: string }>;
};

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

export async function GET(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { alias } = await context.params;
    const svc = getSubscriptionService();
    const value = await svc.getAlias(alias);
    if (!value) {
      return fail("alias not found", 404);
    }
    return NextResponse.json(value);
  } catch (error) {
    return fail("failed to get alias", 400, String(error));
  }
}

export async function PUT(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { alias } = await context.params;
    const body = (await request.json()) as Partial<
      Omit<AliasMapping, "alias" | "updatedAt">
    >;
    const svc = getSubscriptionService();
    const current = await svc.getAlias(alias);
    if (!current) {
      return fail("alias not found", 404);
    }
    const saved = await svc.upsertAlias({
      alias,
      profileId: body.profileId ?? current.profileId,
      description: body.description ?? current.description,
    });
    return NextResponse.json(saved);
  } catch (error) {
    return fail("failed to update alias", 400, String(error));
  }
}

export async function DELETE(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { alias } = await context.params;
    const svc = getSubscriptionService();
    await svc.deleteAlias(alias);
    return NextResponse.json({ success: true });
  } catch (error) {
    return fail("failed to delete alias", 400, String(error));
  }
}
