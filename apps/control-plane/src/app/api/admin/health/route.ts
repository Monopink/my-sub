import { NextResponse } from "next/server";
import {
  getBackendInfo,
  getSubscriptionService,
} from "@/modules/subscription/interface/container";
import { requireAdminAccess } from "@/modules/subscription/interface/adminAuth";

export async function GET(request: Request) {
  const auth = await requireAdminAccess(request);
  if (!auth.ok) {
    return auth.response;
  }
  const svc = getSubscriptionService();
  const status = await svc.health();
  return NextResponse.json({
    status: "ok",
    schema: status.schema,
    kv_backend: getBackendInfo().kvBackend,
    actor: auth.identity.email,
    at: new Date().toISOString(),
  });
}
