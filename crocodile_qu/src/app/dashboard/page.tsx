import { redirect } from "next/navigation";

import { auth, signOut } from "@/auth";
import { prisma } from "@/lib/prisma";

async function signOutAction() {
  "use server";
  await signOut({ redirectTo: "/login" });
}

export default async function DashboardPage() {
  const session = await auth();

  if (!session?.user) {
    redirect("/login");
  }

  const accounts = await prisma.account.findMany({
    where: {
      userId: session.user.id,
    },
    orderBy: {
      provider: "asc",
    },
  });

  return (
    <main className="min-h-screen py-8 md:py-16">
      <div className="shell grid gap-6 lg:grid-cols-[1.15fr_0.85fr]">
        <section className="glass rounded-[2rem] p-8 shadow-[0_30px_80px_rgba(24,32,38,0.12)] md:p-12">
          <div className="flex flex-wrap items-start justify-between gap-4">
            <div>
              <p className="text-sm uppercase tracking-[0.22em] text-slate-500">
                Authenticated
              </p>
              <h1 className="mt-3 text-4xl font-semibold">
                {session.user.name ?? session.user.email ?? "匿名用户"}
              </h1>
              <p className="mt-2 text-slate-600">
                userId: <span className="mono">{session.user.id}</span>
              </p>
            </div>
            <form action={signOutAction}>
              <button
                type="submit"
                className="rounded-full bg-slate-900 px-5 py-3 text-sm font-semibold text-white transition hover:bg-slate-700"
              >
                退出登录
              </button>
            </form>
          </div>

          <div className="mt-10 rounded-[1.5rem] border border-[var(--line)] bg-white/80 p-5">
            <h2 className="text-lg font-semibold">当前会话</h2>
            <dl className="mt-4 grid gap-3 text-sm text-slate-600">
              <div className="flex flex-wrap items-center justify-between gap-3 border-b border-slate-200/70 pb-3">
                <dt>邮箱</dt>
                <dd className="mono">{session.user.email ?? "provider 未返回 email"}</dd>
              </div>
              <div className="flex flex-wrap items-center justify-between gap-3 border-b border-slate-200/70 pb-3">
                <dt>角色</dt>
                <dd className="mono">{session.user.role}</dd>
              </div>
              <div className="flex flex-wrap items-center justify-between gap-3">
                <dt>已关联账号数</dt>
                <dd className="mono">{accounts.length}</dd>
              </div>
            </dl>
          </div>
        </section>

        <aside className="glass rounded-[2rem] p-8 shadow-[0_30px_80px_rgba(24,32,38,0.1)] md:p-10">
          <h2 className="text-2xl font-semibold">已落库的第三方账号</h2>
          <div className="mt-6 space-y-3">
            {accounts.map((account) => (
              <div
                key={`${account.provider}:${account.providerAccountId}`}
                className="rounded-[1.25rem] border border-[var(--line)] bg-white/80 p-4"
              >
                <p className="text-base font-semibold">{account.provider}</p>
                <p className="mono mt-1 text-xs text-slate-500">
                  {account.providerAccountId}
                </p>
              </div>
            ))}
          </div>
        </aside>
      </div>
    </main>
  );
}
