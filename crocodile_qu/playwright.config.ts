import { defineConfig } from "@playwright/test";

const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://localhost:3000";

export default defineConfig({
  testDir: "./e2e",
  fullyParallel: false,
  retries: process.env.CI ? 2 : 0,
  reporter: "list",
  use: {
    baseURL,
    trace: "retain-on-failure",
  },
  webServer: {
    command: "pnpm exec next start --port 3000",
    url: `${baseURL}/login`,
    reuseExistingServer: !process.env.CI,
    env: {
      ...process.env,
      AUTH_SECRET:
        process.env.AUTH_SECRET ??
        "e2e-secret-e2e-secret-e2e-secret-e2e-secret",
      AUTH_URL: baseURL,
      DATABASE_URL:
        process.env.DATABASE_URL ?? "mysql://root:root@127.0.0.1:3307/crocodile_qu",
      E2E_USE_MOCK_OAUTH: "true",
    },
  },
});
