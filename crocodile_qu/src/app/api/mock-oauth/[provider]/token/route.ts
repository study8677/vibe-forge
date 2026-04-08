import { NextRequest, NextResponse } from "next/server";

import { assertMockOAuthEnabled, decodeMockToken } from "@/lib/mock-oauth";

type RouteContext = {
  params: Promise<{
    provider: string;
  }>;
};

export async function POST(request: NextRequest, context: RouteContext) {
  assertMockOAuthEnabled();

  const { provider } = await context.params;
  const body = await request.text();
  const searchParams = new URLSearchParams(body);
  const code = searchParams.get("code");

  if (!code) {
    return NextResponse.json({ error: "code is required" }, { status: 400 });
  }

  const payload = decodeMockToken(code);
  if (payload.providerId !== provider) {
    return NextResponse.json({ error: "provider mismatch" }, { status: 400 });
  }

  return NextResponse.json({
    access_token: code,
    token_type: "bearer",
    expires_in: 3600,
    refresh_token: `${code}.refresh`,
    uid: payload.subject,
  });
}
