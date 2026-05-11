import { NextResponse } from "next/server";
import type { Profile } from "@/modules/subscription/domain/entities";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

type RouteContext = {
  params: Promise<{ profileId: string }>;
};

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

export async function GET(request: Request, context: RouteContext) {
  const auth = await requireAdminAccess(request);
  if (!auth.ok) {
    return auth.response;
  }
  const { profileId } = await context.params;
  const svc = getSubscriptionService();
  const profile = await svc.getProfile(profileId);
  if (!profile) {
    return fail("profile not found", 404);
  }
  return NextResponse.json(profile);
}

export async function PUT(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { profileId } = await context.params;
    const body = (await request.json()) as Partial<Profile>;
    const svc = getSubscriptionService();
    const saved = await svc.upsertProfile({
      id: profileId,
      ...body,
    });
    return NextResponse.json(saved);
  } catch (error) {
    return fail("failed to update profile", 400, String(error));
  }
}

export async function DELETE(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { profileId } = await context.params;
    const svc = getSubscriptionService();
    const aliases = await svc.listAliases();
    const linkedAliases = aliases
      .filter((item) => item.profileId === profileId)
      .map((item) => item.alias);
    if (linkedAliases.length > 0) {
      return fail("profile is referenced by aliases", 409, {
        profileId,
        aliases: linkedAliases,
      });
    }
    await svc.deleteProfile(profileId);
    return NextResponse.json({ success: true });
  } catch (error) {
    return fail("failed to delete profile", 400, String(error));
  }
}
