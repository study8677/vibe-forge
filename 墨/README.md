# 墨 · 工具集

> **提示词:** 开发一个工具网站，包含常见的加解密操作、图片压缩裁剪等操作、页面风格为黑白科技风，总工具不低于20个。

## 项目简介

黑白科技风的纯前端工具箱网站，内置 26 个常用开发工具，覆盖编解码、加密解密、文本处理、图片操作和开发辅助五大类。无需后端，浏览器打开即用。

## 核心功能

- **编解码 (5)**: Base64、URL、HTML 实体、Unicode、Hex 编解码
- **加密解密 (6)**: MD5、SHA、HMAC、AES 加解密、RSA 密钥生成、JWT 解析
- **文本工具 (6)**: JSON/XML 格式化、正则测试器、文本 Diff、字数统计、Markdown 预览
- **图片工具 (5)**: 图片压缩、裁剪、格式转换、Base64 互转、二维码生成
- **开发工具 (4)**: UUID 生成、时间戳转换、颜色转换、进制转换

## 技术栈

| 层级 | 技术 |
|------|------|
| 结构 | HTML5 |
| 样式 | CSS3 (JetBrains Mono / 网格背景 / 扫描线) |
| 逻辑 | Vanilla JavaScript (ES6+) |
| 加密 | CryptoJS 4.2 (CDN) |
| 二维码 | qrcode-generator 1.4 (CDN) |

## 文件说明

- `index.html` — 主入口，包含页面骨架与 CDN 引用
- `css/style.css` — 黑白科技风主题样式
- `js/app.js` — 核心路由、搜索、分类筛选、工具注册
- `js/tools/encoding.js` — 5 个编解码工具
- `js/tools/crypto.js` — 6 个加密解密工具
- `js/tools/text.js` — 6 个文本处理工具
- `js/tools/image.js` — 5 个图片操作工具
- `js/tools/dev.js` — 4 个开发辅助工具

## 运行方式

浏览器直接打开 `index.html` 即可（需联网加载 CryptoJS 与 qrcode-generator CDN）。
