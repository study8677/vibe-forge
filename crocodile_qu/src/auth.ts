import NextAuth from "next-auth";
import { PrismaAdapter } from "@auth/prisma-adapter";

import { buildProviders } from "@/auth/providers";
import { prisma } from "@/lib/prisma";

export const { handlers, auth, signIn, signOut } = NextAuth({
  adapter: PrismaAdapter(prisma),
  session: {
    strategy: "database",
  },
  trustHost: true,
  pages: {
    signIn: "/login",
  },
  providers: buildProviders(),
  callbacks: {
    session({ session, user }) {
      if (session.user) {
        session.user.id = user.id;
        session.user.role = user.role ?? "user";
      }

      return session;
    },
  },
});
