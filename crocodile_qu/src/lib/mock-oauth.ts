import { Buffer } from "node:buffer";

type MockAuthorizationPayload = {
  providerId: string;
  subject: string;
  name: string;
  email: string;
  picture: string;
};

function isMockOAuthEnabled(): boolean {
  return process.env.E2E_USE_MOCK_OAUTH === "true";
}

export function assertMockOAuthEnabled(): void {
  if (!isMockOAuthEnabled()) {
    throw new Error("Mock OAuth routes are disabled outside E2E_USE_MOCK_OAUTH=true");
  }
}

export function buildMockIdentity(providerId: string): MockAuthorizationPayload {
  return {
    providerId,
    subject: `${providerId}-demo-user`,
    name: `${providerId.toUpperCase()} Demo User`,
    email: `${providerId}.demo@example.com`,
    picture: `https://avatars.example.test/${providerId}.png`,
  };
}

export function encodeMockToken(payload: MockAuthorizationPayload): string {
  return Buffer.from(JSON.stringify(payload)).toString("base64url");
}

export function decodeMockToken(token: string): MockAuthorizationPayload {
  return JSON.parse(Buffer.from(token, "base64url").toString("utf8")) as MockAuthorizationPayload;
}

export function renderMockAuthorizePage(args: {
  providerId: string;
  callbackUrl: string;
  state: string;
}): string {
  const approveUrl = new URL(args.callbackUrl);
  approveUrl.searchParams.set(
    "code",
    encodeMockToken(buildMockIdentity(args.providerId)),
  );
  approveUrl.searchParams.set("state", args.state);

  return `<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Mock OAuth - ${args.providerId}</title>
    <style>
      body {
        margin: 0;
        font-family: ui-sans-serif, system-ui, sans-serif;
        background: #0f172a;
        color: #e2e8f0;
      }
      main {
        max-width: 680px;
        margin: 8rem auto;
        padding: 2rem;
        border-radius: 24px;
        background: linear-gradient(180deg, #111827 0%, #0f172a 100%);
        box-shadow: 0 24px 80px rgba(15, 23, 42, 0.45);
      }
      h1 {
        margin-top: 0;
        font-size: 2rem;
      }
      p {
        line-height: 1.7;
        color: #cbd5e1;
      }
      a {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        min-width: 12rem;
        padding: 0.875rem 1.25rem;
        border-radius: 999px;
        background: #22c55e;
        color: #04130a;
        text-decoration: none;
        font-weight: 700;
      }
    </style>
  </head>
  <body>
    <main>
      <h1>Mock OAuth 授权页</h1>
      <p>Provider: <strong>${args.providerId}</strong></p>
      <p>这个页面仅用于 Playwright E2E 测试，点击下面按钮会模拟第三方授权并回跳到应用。</p>
      <a href="${approveUrl.toString()}">Approve mock sign-in</a>
    </main>
  </body>
</html>`;
}
