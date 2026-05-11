import { NextResponse } from "next/server";
import {
  getBackendInfo,
  getSubscriptionService,
} from "@/modules/subscription/interface/container";

export async function GET() {
  const svc = getSubscriptionService();
  const status = await svc.health();
  return NextResponse.json({
    status: "ok",
    schema: status.schema,
    kv_backend: getBackendInfo().kvBackend,
    at: new Date().toISOString(),
  });
}

