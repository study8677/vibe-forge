import { NextRequest, NextResponse } from "next/server";

import { assertMockOAuthEnabled, decodeMockToken } from "@/lib/mock-oauth";

type RouteContext = {
  params: Promise<{
    provider: string;
  }>;
};

function getAccessToken(request: NextRequest): string | null {
  const authorizationHeader = request.headers.get("authorization");
  if (authorizationHeader?.startsWith("Bearer ")) {
    return authorizationHeader.slice("Bearer ".length);
  }

  return request.nextUrl.searchParams.get("access_token");
}

export async function GET(request: NextRequest, context: RouteContext) {
  assertMockOAuthEnabled();

  const { provider } = await context.params;
  const accessToken = getAccessToken(request);

  if (!accessToken) {
    return NextResponse.json({ error: "access_token is required" }, { status: 400 });
  }

  const payload = decodeMockToken(accessToken);
  if (payload.providerId !== provider) {
    return NextResponse.json({ error: "provider mismatch" }, { status: 400 });
  }

  return NextResponse.json({
    sub: payload.subject,
    id: payload.subject,
    name: payload.name,
    nickname: payload.name,
    email: payload.email,
    picture: payload.picture,
    avatar: payload.picture,
  });
}
