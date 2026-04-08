import type { OAuthConfig, Provider } from "next-auth/providers";
import GitHub from "next-auth/providers/github";
import MicrosoftEntraID from "next-auth/providers/microsoft-entra-id";
import WeChat from "next-auth/providers/wechat";

type GenericProfile = Record<string, unknown>;

type SupportedProviderId =
  | "github"
  | "microsoft-entra-id"
  | "wechat"
  | "qq"
  | "weibo"
  | "tencent"
  | "alibaba"
  | "bytedance";

type ProviderDescriptor = {
  id: SupportedProviderId;
  name: string;
  buttonLabel: string;
  description: string;
};

type GenericProviderOptions = {
  id: "tencent" | "alibaba" | "bytedance";
  name: string;
  clientId: string;
  clientSecret: string;
  authorizationUrl: string;
  tokenUrl: string;
  userinfoUrl: string;
  scope?: string;
  userIdField?: string;
  userNameField?: string;
  userEmailField?: string;
  userImageField?: string;
};

type MockProfile = {
  sub: string;
  name: string;
  email: string;
  picture: string;
};

type AppTokenSet = Record<string, unknown> & {
  access_token?: string;
  expires_at?: number;
  token_type?: string;
  uid?: string;
};

type TokenRequestContext = {
  params: Record<string, string | number | undefined>;
  provider: {
    clientId: string;
    clientSecret: string;
    callbackUrl: string;
  };
};

type UserinfoRequestContext = {
  tokens: AppTokenSet;
  provider: {
    clientId: string;
    userinfo?: {
      url: URL;
    };
  };
};

const PROVIDER_CATALOG: ProviderDescriptor[] = [
  {
    id: "github",
    name: "GitHub",
    buttonLabel: "继续使用 GitHub 登录",
    description: "内置 GitHub OAuth provider。",
  },
  {
    id: "microsoft-entra-id",
    name: "Outlook / Microsoft",
    buttonLabel: "继续使用 Outlook 登录",
    description: "通过 Microsoft Entra ID 覆盖 Outlook / Microsoft 账号登录。",
  },
  {
    id: "wechat",
    name: "微信",
    buttonLabel: "继续使用微信登录",
    description: "使用 Auth.js 内置 WeChat provider。",
  },
  {
    id: "qq",
    name: "QQ",
    buttonLabel: "继续使用 QQ 登录",
    description: "自定义 QQ OAuth provider，补齐 openid 查询。",
  },
  {
    id: "weibo",
    name: "微博",
    buttonLabel: "继续使用微博登录",
    description: "自定义微博 OAuth provider。",
  },
  {
    id: "tencent",
    name: "腾讯开放平台",
    buttonLabel: "继续使用腾讯开放平台登录",
    description: "通用 OAuth 插槽，适配未明确的腾讯账号体系。",
  },
  {
    id: "alibaba",
    name: "阿里开放平台",
    buttonLabel: "继续使用阿里开放平台登录",
    description: "通用 OAuth 插槽，适配未明确的阿里账号体系。",
  },
  {
    id: "bytedance",
    name: "字节开放平台",
    buttonLabel: "继续使用字节开放平台登录",
    description: "通用 OAuth 插槽，适配未明确的字节账号体系。",
  },
];

function hasValue(value: string | undefined): value is string {
  return Boolean(value && value.trim().length > 0);
}

function requiredEnv(key: string): string {
  const value = process.env[key];
  if (!hasValue(value)) {
    throw new Error(`Missing required environment variable: ${key}`);
  }
  return value;
}

function getBaseUrl(): string {
  return (
    process.env.AUTH_URL ??
    process.env.NEXTAUTH_URL ??
    "http://127.0.0.1:3000"
  );
}

function isMockOAuthEnabled(): boolean {
  return process.env.E2E_USE_MOCK_OAUTH === "true";
}

function pickString(
  source: Record<string, unknown>,
  key: string | undefined,
): string | null {
  if (!key) {
    return null;
  }

  const value = source[key];
  return typeof value === "string" && value.length > 0 ? value : null;
}

function buildMockProvider(descriptor: ProviderDescriptor): OAuthConfig<MockProfile> {
  const baseUrl = getBaseUrl();

  return {
    id: descriptor.id,
    name: descriptor.name,
    type: "oauth",
    clientId: `${descriptor.id}-mock-client-id`,
    clientSecret: `${descriptor.id}-mock-client-secret`,
    checks: ["state"],
    authorization: {
      url: `${baseUrl}/api/mock-oauth/${descriptor.id}/authorize`,
      params: {
        response_type: "code",
        scope: "profile email",
      },
    },
    token: {
      url: `${baseUrl}/api/mock-oauth/${descriptor.id}/token`,
    },
    userinfo: {
      url: `${baseUrl}/api/mock-oauth/${descriptor.id}/userinfo`,
    },
    profile(profile: MockProfile) {
      return {
        id: profile.sub,
        name: profile.name,
        email: profile.email,
        image: profile.picture,
      };
    },
  };
}

function buildQQProvider(): OAuthConfig<GenericProfile> {
  return {
    id: "qq",
    name: "QQ",
    type: "oauth",
    clientId: requiredEnv("AUTH_QQ_ID"),
    clientSecret: requiredEnv("AUTH_QQ_SECRET"),
    checks: ["state"],
    authorization: {
      url: "https://graph.qq.com/oauth2.0/authorize",
      params: {
        response_type: "code",
        scope: "get_user_info",
      },
    },
    token: {
      url: "https://graph.qq.com/oauth2.0/token",
      async request({ params, provider }: TokenRequestContext) {
        const tokenUrl = new URL("https://graph.qq.com/oauth2.0/token");
        tokenUrl.searchParams.set("grant_type", "authorization_code");
        tokenUrl.searchParams.set("client_id", provider.clientId);
        tokenUrl.searchParams.set("client_secret", provider.clientSecret);
        tokenUrl.searchParams.set("code", String(params.code ?? ""));
        tokenUrl.searchParams.set("redirect_uri", provider.callbackUrl);

        const response = await fetch(tokenUrl);
        const text = await response.text();

        if (!response.ok) {
          throw new Error(`QQ token exchange failed: ${text}`);
        }

        const searchParams = new URLSearchParams(text);
        const accessToken = searchParams.get("access_token");
        const expiresIn = Number(searchParams.get("expires_in") ?? "0");

        if (!accessToken) {
          throw new Error(`QQ token exchange did not return access_token: ${text}`);
        }

        return {
          tokens: {
            access_token: accessToken,
            expires_at:
              expiresIn > 0 ? Math.floor(Date.now() / 1000) + expiresIn : undefined,
            token_type: "bearer",
          } as AppTokenSet,
        };
      },
    },
    userinfo: {
      url: "https://graph.qq.com/user/get_user_info",
      async request({ tokens, provider }: UserinfoRequestContext) {
        const accessToken = tokens.access_token;
        if (!accessToken) {
          throw new Error("QQ userinfo is missing access_token");
        }

        const meResponse = await fetch(
          `https://graph.qq.com/oauth2.0/me?access_token=${encodeURIComponent(
            accessToken,
          )}`,
        );
        const meText = await meResponse.text();
        const openIdMatch = meText.match(/"openid"\s*:\s*"([^"]+)"/);
        const openid = openIdMatch?.[1];

        if (!openid) {
          throw new Error(`QQ openid lookup failed: ${meText}`);
        }

        const profileUrl = new URL(provider.userinfo?.url ?? "");
        profileUrl.searchParams.set("access_token", accessToken);
        profileUrl.searchParams.set("oauth_consumer_key", provider.clientId);
        profileUrl.searchParams.set("openid", openid);

        const profileResponse = await fetch(profileUrl);
        const profile = (await profileResponse.json()) as GenericProfile;

        return {
          ...profile,
          openid,
        };
      },
    },
    profile(profile: GenericProfile) {
      const openid = pickString(profile, "openid");
      if (!openid) {
        throw new Error("QQ profile is missing openid");
      }

      return {
        id: openid,
        name: pickString(profile, "nickname") ?? "QQ User",
        email: null,
        image:
          pickString(profile, "figureurl_qq_2") ??
          pickString(profile, "figureurl_2") ??
          null,
      };
    },
  };
}

function buildWeiboProvider(): OAuthConfig<GenericProfile> {
  return {
    id: "weibo",
    name: "微博",
    type: "oauth",
    clientId: requiredEnv("AUTH_WEIBO_ID"),
    clientSecret: requiredEnv("AUTH_WEIBO_SECRET"),
    checks: ["state"],
    client: {
      token_endpoint_auth_method: "client_secret_post",
    },
    authorization: {
      url: "https://api.weibo.com/oauth2/authorize",
      params: {
        response_type: "code",
      },
    },
    token: {
      url: "https://api.weibo.com/oauth2/access_token",
    },
    userinfo: {
      url: "https://api.weibo.com/2/users/show.json",
      async request({ tokens, provider }: UserinfoRequestContext) {
        const accessToken = tokens.access_token;
        const uid = typeof tokens.uid === "string" ? tokens.uid : null;

        if (!accessToken || !uid) {
          throw new Error("Weibo userinfo is missing access_token or uid");
        }

        const profileUrl = new URL(provider.userinfo?.url ?? "");
        profileUrl.searchParams.set("access_token", accessToken);
        profileUrl.searchParams.set("uid", uid);

        const response = await fetch(profileUrl);
        return (await response.json()) as GenericProfile;
      },
    },
    profile(profile: GenericProfile) {
      const id =
        pickString(profile, "idstr") ??
        (typeof profile.id === "number" ? String(profile.id) : null);

      if (!id) {
        throw new Error("Weibo profile is missing id");
      }

      return {
        id,
        name: pickString(profile, "screen_name") ?? "Weibo User",
        email: null,
        image:
          pickString(profile, "avatar_large") ??
          pickString(profile, "profile_image_url") ??
          null,
      };
    },
  };
}

function buildGenericProvider(options: GenericProviderOptions): OAuthConfig<GenericProfile> {
  return {
    id: options.id,
    name: options.name,
    type: "oauth",
    clientId: options.clientId,
    clientSecret: options.clientSecret,
    checks: ["state"],
    client: {
      token_endpoint_auth_method: "client_secret_post",
    },
    authorization: {
      url: options.authorizationUrl,
      params: {
        response_type: "code",
        scope: options.scope ?? "openid profile email",
      },
    },
    token: {
      url: options.tokenUrl,
    },
    userinfo: {
      url: options.userinfoUrl,
    },
    profile(profile: GenericProfile) {
      const id =
        pickString(profile, options.userIdField) ??
        pickString(profile, "sub") ??
        pickString(profile, "id");

      if (!id) {
        throw new Error(`${options.name} profile is missing a user id field`);
      }

      return {
        id,
        name:
          pickString(profile, options.userNameField) ??
          pickString(profile, "name") ??
          pickString(profile, "nickname") ??
          options.name,
        email:
          pickString(profile, options.userEmailField) ??
          pickString(profile, "email") ??
          null,
        image:
          pickString(profile, options.userImageField) ??
          pickString(profile, "picture") ??
          pickString(profile, "avatar") ??
          null,
      };
    },
  };
}

function readGenericProviderEnv(
  prefix: "AUTH_TENCENT" | "AUTH_ALIBABA" | "AUTH_BYTEDANCE",
  id: GenericProviderOptions["id"],
  name: string,
): GenericProviderOptions | null {
  const clientId = process.env[`${prefix}_ID`];
  const clientSecret = process.env[`${prefix}_SECRET`];
  const authorizationUrl = process.env[`${prefix}_AUTHORIZATION_URL`];
  const tokenUrl = process.env[`${prefix}_TOKEN_URL`];
  const userinfoUrl = process.env[`${prefix}_USERINFO_URL`];

  if (
    !hasValue(clientId) ||
    !hasValue(clientSecret) ||
    !hasValue(authorizationUrl) ||
    !hasValue(tokenUrl) ||
    !hasValue(userinfoUrl)
  ) {
    return null;
  }

  return {
    id,
    name,
    clientId,
    clientSecret,
    authorizationUrl,
    tokenUrl,
    userinfoUrl,
    scope: process.env[`${prefix}_SCOPE`],
    userIdField: process.env[`${prefix}_USER_ID_FIELD`] ?? "sub",
    userNameField: process.env[`${prefix}_USER_NAME_FIELD`] ?? "name",
    userEmailField: process.env[`${prefix}_USER_EMAIL_FIELD`] ?? "email",
    userImageField: process.env[`${prefix}_USER_IMAGE_FIELD`] ?? "picture",
  };
}

function isEnabled(descriptor: ProviderDescriptor): boolean {
  if (isMockOAuthEnabled()) {
    return true;
  }

  switch (descriptor.id) {
    case "github":
      return hasValue(process.env.AUTH_GITHUB_ID) && hasValue(process.env.AUTH_GITHUB_SECRET);
    case "microsoft-entra-id":
      return (
        hasValue(process.env.AUTH_MICROSOFT_ENTRA_ID_ID) &&
        hasValue(process.env.AUTH_MICROSOFT_ENTRA_ID_SECRET)
      );
    case "wechat":
      return hasValue(process.env.AUTH_WECHAT_ID) && hasValue(process.env.AUTH_WECHAT_SECRET);
    case "qq":
      return hasValue(process.env.AUTH_QQ_ID) && hasValue(process.env.AUTH_QQ_SECRET);
    case "weibo":
      return hasValue(process.env.AUTH_WEIBO_ID) && hasValue(process.env.AUTH_WEIBO_SECRET);
    case "tencent":
      return Boolean(readGenericProviderEnv("AUTH_TENCENT", "tencent", "腾讯开放平台"));
    case "alibaba":
      return Boolean(readGenericProviderEnv("AUTH_ALIBABA", "alibaba", "阿里开放平台"));
    case "bytedance":
      return Boolean(readGenericProviderEnv("AUTH_BYTEDANCE", "bytedance", "字节开放平台"));
  }
}

function buildProvider(descriptor: ProviderDescriptor): Provider | null {
  if (!isEnabled(descriptor)) {
    return null;
  }

  if (isMockOAuthEnabled()) {
    return buildMockProvider(descriptor);
  }

  switch (descriptor.id) {
    case "github":
      return GitHub({
        clientId: requiredEnv("AUTH_GITHUB_ID"),
        clientSecret: requiredEnv("AUTH_GITHUB_SECRET"),
        name: descriptor.name,
      });
    case "microsoft-entra-id":
      return MicrosoftEntraID({
        clientId: requiredEnv("AUTH_MICROSOFT_ENTRA_ID_ID"),
        clientSecret: requiredEnv("AUTH_MICROSOFT_ENTRA_ID_SECRET"),
        issuer: process.env.AUTH_MICROSOFT_ENTRA_ID_ISSUER,
        name: descriptor.name,
      });
    case "wechat":
      return WeChat({
        clientId: requiredEnv("AUTH_WECHAT_ID"),
        clientSecret: requiredEnv("AUTH_WECHAT_SECRET"),
        platformType:
          process.env.AUTH_WECHAT_PLATFORM_TYPE === "OfficialAccount"
            ? "OfficialAccount"
            : "WebsiteApp",
      });
    case "qq":
      return buildQQProvider();
    case "weibo":
      return buildWeiboProvider();
    case "tencent": {
      const options = readGenericProviderEnv(
        "AUTH_TENCENT",
        "tencent",
        "腾讯开放平台",
      );
      return options ? buildGenericProvider(options) : null;
    }
    case "alibaba": {
      const options = readGenericProviderEnv(
        "AUTH_ALIBABA",
        "alibaba",
        "阿里开放平台",
      );
      return options ? buildGenericProvider(options) : null;
    }
    case "bytedance": {
      const options = readGenericProviderEnv(
        "AUTH_BYTEDANCE",
        "bytedance",
        "字节开放平台",
      );
      return options ? buildGenericProvider(options) : null;
    }
  }
}

export function getEnabledProviderDescriptors(): ProviderDescriptor[] {
  return PROVIDER_CATALOG.filter((descriptor) => isEnabled(descriptor));
}

export function buildProviders(): Provider[] {
  return PROVIDER_CATALOG.map((descriptor) => buildProvider(descriptor)).filter(
    (provider): provider is Provider => provider !== null,
  );
}
