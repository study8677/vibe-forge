# 局域网媒体站 (Media Station)

> **提示词:** 开发一个轻量高效局域网媒体应用，类似群晖Photo/Video Station。支持千万级文件，高性能数据库，照片自动生成缩略图。

## 项目简介

轻量级局域网媒体管理应用，对标群晖 Photo Station / Video Station。编译为单一可执行文件，零外部依赖，启动即用。核心设计面向千万级文件规模，SQLite WAL 模式 + 64MB 缓存 + 256MB 内存映射确保高性能查询与索引。

## 核心功能

- **高速文件扫描** — 递归遍历多个媒体目录，5000 条/事务批量入库，原子计数器追踪进度
- **自动缩略图生成** — 后台 rayon 并行处理，生成 240px / 720px 双尺寸缩略图，EXIF 自动旋转
- **EXIF 元数据提取** — 自动解析拍摄日期、方向信息，支持按时间线浏览
- **流式文件传输** — 原始文件流式响应 + HTTP Range 支持，大视频无需全量加载即可拖拽播放
- **暗色主题 Web UI** — Gallery / Timeline / Folders 三种视图，灯箱预览，无限滚动

## 技术栈

| 层级 | 技术 |
|------|------|
| 后端框架 | Rust + Axum 0.7 (async) |
| 数据库 | SQLite WAL (rusqlite, bundled) |
| 图像处理 | image crate + rayon 并行 |
| EXIF 解析 | kamadak-exif |
| 前端 | 原生 HTML / CSS / JavaScript (嵌入二进制) |

## 文件说明

- `src/main.rs` — 入口：初始化数据库、启动扫描/缩略图线程、Web 服务器
- `src/config.rs` — CLI 参数定义 (clap)
- `src/db.rs` — SQLite 高性能数据层 (WAL + 批量写入 + 动态查询)
- `src/scanner.rs` — 递归文件扫描器 (walkdir + 原子进度计数)
- `src/thumbnail.rs` — 后台缩略图生成 (rayon 并行 + 65536 级目录分桶)
- `src/exif_parser.rs` — EXIF 日期/方向提取
- `src/api.rs` — REST API + 静态资源嵌入 + HTTP Range 视频流
- `src/models.rs` — 共享数据模型
- `static/` — 前端 SPA (暗色主题, 响应式 Grid 布局)

## 运行方式

```bash
# 安装 Rust: https://rustup.rs

# 编译
cargo build --release

# 启动 (指定一个或多个媒体目录)
./target/release/media-station -d /path/to/photos -d /path/to/videos

# 自定义端口和数据目录
./target/release/media-station -d /photos --port 8080 --data-dir ./mydata
```

启动后浏览器访问 `http://localhost:9110`，自动扫描并生成缩略图。
