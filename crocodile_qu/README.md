# Crocodile OAuth Hub

一个基于 `Next.js + Auth.js + Prisma + MySQL + Playwright` 的多第三方登录集成样板。

## 已覆盖的 provider

- `GitHub`
- `Outlook / Microsoft`：通过 `Microsoft Entra ID`
- `微信`：使用 Auth.js 内置 `WeChat` provider
- `QQ`：自定义 OAuth provider，补齐 `openid` 查询
- `微博`：自定义 OAuth provider
- `腾讯 / 阿里 / 字节`：提供通用 OAuth 插槽，通过环境变量绑定到具体开放平台

## 设计取舍

- 真实第三方平台登录和 E2E 测试解耦。E2E 使用本地 mock OAuth 服务，不依赖外部平台可用性、审核状态或测试账号。
- 对明确支持的 provider 用内置配置；对 Auth.js 当前未内置的 provider 走自定义 OAuth；对用户未明确到具体产品线的平台保留通用插槽。
- Session 使用 MySQL 持久化，用户与第三方账号关系落在 Prisma 的 `User / Account / Session / VerificationToken` 模型上。

## 本地启动

1. 复制环境变量文件：

```bash
cp .env.example .env.local
```

2. 启动 MySQL：

```bash
pnpm db:up
pnpm db:wait
```

3. 推送 Prisma schema：

```bash
pnpm db:push
pnpm db:generate
```

4. 启动开发服务器：

```bash
pnpm dev
```

打开 `http://127.0.0.1:3000/login`。

## E2E 测试

先安装 Playwright 浏览器：

```bash
pnpm exec playwright install chromium
```

确保 MySQL 已启动并且 `DATABASE_URL` 指向可写的 MySQL 库，然后运行：

```bash
pnpm test:e2e
```

Playwright 会自动以 `E2E_USE_MOCK_OAUTH=true` 启动应用，并覆盖所有 provider 的授权流程：

- 登录页点击 provider 按钮
- 跳转到 mock 授权页
- 回调 Auth.js
- 落库 `Account`
- 成功进入 `/dashboard`
- 退出登录

## 环境变量

`.env.example` 中已经列出所有所需项。

### 明确 provider

- `AUTH_GITHUB_ID`
- `AUTH_GITHUB_SECRET`
- `AUTH_MICROSOFT_ENTRA_ID_ID`
- `AUTH_MICROSOFT_ENTRA_ID_SECRET`
- `AUTH_MICROSOFT_ENTRA_ID_ISSUER`
- `AUTH_WECHAT_ID`
- `AUTH_WECHAT_SECRET`
- `AUTH_WECHAT_PLATFORM_TYPE`
- `AUTH_QQ_ID`
- `AUTH_QQ_SECRET`
- `AUTH_WEIBO_ID`
- `AUTH_WEIBO_SECRET`

### 通用平台插槽

下列三组通过标准 OAuth2 / OIDC 端点接入：

- `AUTH_TENCENT_*`
- `AUTH_ALIBABA_*`
- `AUTH_BYTEDANCE_*`

每组至少需要：

- `*_ID`
- `*_SECRET`
- `*_AUTHORIZATION_URL`
- `*_TOKEN_URL`
- `*_USERINFO_URL`

如果 userinfo 字段名不是标准的 `sub / name / email / picture`，可以通过：

- `*_USER_ID_FIELD`
- `*_USER_NAME_FIELD`
- `*_USER_EMAIL_FIELD`
- `*_USER_IMAGE_FIELD`

重映射。
