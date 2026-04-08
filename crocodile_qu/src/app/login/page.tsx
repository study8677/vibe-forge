import { redirect } from "next/navigation";

import { auth, signIn } from "@/auth";
import { getEnabledProviderDescriptors } from "@/auth/providers";

async function signInAction(formData: FormData) {
  "use server";

  const providerId = formData.get("provider");
  if (typeof providerId !== "string" || providerId.length === 0) {
    return;
  }

  await signIn(providerId, { redirectTo: "/dashboard" });
}

export default async function LoginPage() {
  const session = await auth();
  if (session?.user) {
    redirect("/dashboard");
  }

  const providers = getEnabledProviderDescriptors();
  const isMockMode = process.env.E2E_USE_MOCK_OAUTH === "true";

  return (
    <main className="min-h-screen py-8 md:py-16">
      <div className="shell grid gap-6 lg:grid-cols-[1.15fr_0.85fr]">
        <section className="glass overflow-hidden rounded-[2rem] p-8 shadow-[0_30px_80px_rgba(24,32,38,0.12)] md:p-12">
          <div className="mb-8 inline-flex rounded-full border border-[var(--line)] bg-white/60 px-4 py-2 text-sm font-medium text-slate-600">
            Multi OAuth Gateway
          </div>
          <h1 className="hero-title max-w-3xl text-4xl font-semibold leading-tight md:text-6xl">
            把第三方登录接成一个统一入口，而不是一堆散碎按钮。
          </h1>
          <p className="mt-6 max-w-2xl text-lg leading-8 text-slate-600">
            当前实现同时覆盖内置 provider、定制 OAuth provider 和通用平台插槽，
            底层用 Auth.js + Prisma + MySQL 持久化，Playwright E2E 使用本地 mock OAuth
            保证可重复测试。
          </p>

          <div className="mt-10 grid gap-3 md:grid-cols-2">
            {providers.map((provider) => (
              <form key={provider.id} action={signInAction}>
                <input type="hidden" name="provider" value={provider.id} />
                <button
                  type="submit"
                  data-testid={`signin-${provider.id}`}
                  className="flex min-h-16 w-full items-center justify-between rounded-[1.25rem] border border-[var(--line)] bg-white px-5 text-left text-base font-semibold transition hover:-translate-y-0.5 hover:border-slate-400 hover:shadow-[0_18px_35px_rgba(15,23,42,0.08)]"
                >
                  <span>{provider.buttonLabel}</span>
                  <span className="mono text-xs uppercase tracking-[0.2em] text-slate-400">
                    {provider.id}
                  </span>
                </button>
              </form>
            ))}
          </div>
        </section>

        <aside className="glass rounded-[2rem] p-8 shadow-[0_30px_80px_rgba(24,32,38,0.1)] md:p-10">
          <h2 className="text-2xl font-semibold">接入说明</h2>
          <div className="mt-6 space-y-4 text-sm leading-7 text-slate-600">
            <p>
              已启用 provider 数量：<strong>{providers.length}</strong>
            </p>
            <p>
              Mock OAuth 模式：<strong>{isMockMode ? "开启" : "关闭"}</strong>
            </p>
            <p>
              `GitHub / Outlook / 微信` 走 Auth.js 原生 provider；`QQ / 微博`
              走自定义 OAuth；`腾讯 / 阿里 / 字节` 走通用 OAuth 插槽，由环境变量绑定到你
              的实际开放平台。
            </p>
            <p>
              没有配置任何 provider 时，这个页面会是空列表；复制 `.env.example` 到
              `.env.local` 后按需填写即可。
            </p>
          </div>
        </aside>
      </div>
    </main>
  );
}
