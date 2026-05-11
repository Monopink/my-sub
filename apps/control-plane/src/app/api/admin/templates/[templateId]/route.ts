import { NextResponse } from "next/server";
import type { Template } from "@/modules/subscription/domain/entities";
import { getSubscriptionService } from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

type RouteContext = {
  params: Promise<{ templateId: string }>;
};

function fail(message: string, status = 400, details: unknown = null): NextResponse {
  return NextResponse.json({ error: message, details }, { status });
}

export async function GET(request: Request, context: RouteContext) {
  const auth = await requireAdminAccess(request);
  if (!auth.ok) {
    return auth.response;
  }
  const { templateId } = await context.params;
  const svc = getSubscriptionService();
  const template = await svc.getTemplate(templateId);
  if (!template) {
    return fail("template not found", 404);
  }
  return NextResponse.json(template);
}

export async function PUT(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { templateId } = await context.params;
    const body = (await request.json()) as Partial<Template>;
    const svc = getSubscriptionService();
    const saved = await svc.upsertTemplate({
      id: templateId,
      ...body,
    });
    return NextResponse.json(saved);
  } catch (error) {
    return fail("failed to update template", 400, String(error));
  }
}

export async function DELETE(request: Request, context: RouteContext) {
  try {
    const auth = await requireAdminAccess(request);
    if (!auth.ok) {
      return auth.response;
    }
    const { templateId } = await context.params;
    const svc = getSubscriptionService();
    const profiles = await svc.listProfiles();
    const linkedProfiles = profiles
      .filter((item) => item.templateId === templateId)
      .map((item) => item.id);
    if (linkedProfiles.length > 0) {
      return fail("template is referenced by profiles", 409, {
        templateId,
        profiles: linkedProfiles,
      });
    }
    await svc.deleteTemplate(templateId);
    return NextResponse.json({ success: true });
  } catch (error) {
    return fail("failed to delete template", 400, String(error));
  }
}
