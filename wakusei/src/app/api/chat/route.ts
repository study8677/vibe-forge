import { NextRequest } from "next/server";

export const runtime = "edge";

export async function POST(req: NextRequest) {
  const { messages, apiUrl, apiKey, model } = await req.json();

  if (!apiUrl || !apiKey) {
    return new Response(
      JSON.stringify({ error: "API URL and Key are required" }),
      { status: 400, headers: { "Content-Type": "application/json" } }
    );
  }

  // Normalize: ensure the URL ends with /chat/completions
  let endpoint = apiUrl.replace(/\/+$/, "");
  if (!endpoint.endsWith("/chat/completions")) {
    if (!endpoint.endsWith("/v1")) {
      endpoint += "/v1";
    }
    endpoint += "/chat/completions";
  }

  const body = JSON.stringify({
    model: model || "gpt-4o-mini",
    messages,
    stream: true,
  });

  const upstream = await fetch(endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${apiKey}`,
    },
    body,
  });

  if (!upstream.ok) {
    const text = await upstream.text();
    return new Response(
      JSON.stringify({ error: `Upstream ${upstream.status}: ${text}` }),
      { status: upstream.status, headers: { "Content-Type": "application/json" } }
    );
  }

  // Pipe the SSE stream through
  return new Response(upstream.body, {
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
    },
  });
}
