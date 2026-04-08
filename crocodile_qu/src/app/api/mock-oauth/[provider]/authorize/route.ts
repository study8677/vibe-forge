import { NextRequest, NextResponse } from "next/server";

import { assertMockOAuthEnabled, renderMockAuthorizePage } from "@/lib/mock-oauth";

type RouteContext = {
  params: Promise<{
    provider: string;
  }>;
};

export async function GET(request: NextRequest, context: RouteContext) {
  assertMockOAuthEnabled();

  const { provider } = await context.params;
  const callbackUrl = request.nextUrl.searchParams.get("redirect_uri");
  const state = request.nextUrl.searchParams.get("state");

  if (!callbackUrl || !state) {
    return NextResponse.json(
      { error: "redirect_uri and state are required" },
      { status: 400 },
    );
  }

  return new NextResponse(
    renderMockAuthorizePage({
      providerId: provider,
      callbackUrl,
      state,
    }),
    {
      headers: {
        "content-type": "text/html; charset=utf-8",
      },
    },
  );
}
