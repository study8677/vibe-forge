import { expect, test } from "@playwright/test";

const cases = [
  { id: "github", label: "GitHub" },
  { id: "microsoft-entra-id", label: "Outlook" },
  { id: "wechat", label: "微信" },
  { id: "qq", label: "QQ" },
  { id: "weibo", label: "微博" },
  { id: "tencent", label: "腾讯开放平台" },
  { id: "alibaba", label: "阿里开放平台" },
  { id: "bytedance", label: "字节开放平台" },
];

test.describe("third-party login", () => {
  for (const item of cases) {
    test(`can sign in with ${item.id}`, async ({ page }) => {
      await page.goto("/login");

      await page.getByTestId(`signin-${item.id}`).click();
      await expect(page).toHaveURL(
        new RegExp(`/api/mock-oauth/${item.id}/authorize`),
      );

      await page.getByRole("link", { name: "Approve mock sign-in" }).click();
      await expect(page).toHaveURL(/\/dashboard$/);
      await expect(page.getByText(item.id, { exact: true })).toBeVisible();
      await expect(page.getByText(`${item.id}.demo@example.com`)).toBeVisible();

      await page.getByRole("button", { name: "退出登录" }).click();
      await expect(page).toHaveURL(/\/login$/);
    });
  }
});
