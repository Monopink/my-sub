import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

function hasAccessCredential(request: NextRequest): boolean {
  const jwtHeader = request.headers.get("cf-access-jwt-assertion");
  if (jwtHeader && jwtHeader.trim()) {
    return true;
  }
  const accessCookie = request.cookies.get("CF_Authorization")?.value;
  if (accessCookie && accessCookie.trim()) {
    return true;
  }
  return false;
}

export function middleware(request: NextRequest) {
  if (hasAccessCredential(request)) {
    return NextResponse.next();
  }
  return new NextResponse("Not Found", { status: 404 });
}

export const config = {
  matcher: ["/admin/:path*", "/api/admin/:path*"],
};
