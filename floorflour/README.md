# 游戏文本编辑器

> **提示词:** 游戏文本编辑器。文本的修正历史记录。项目化的存储读取。自定义格式，将文本映射或读取自具体程序文件

## 项目简介

FloorFlour 是一个桌面端游戏文本管理工具，用于从各类游戏文件中提取文本、编辑翻译，并将修改写回原文件。支持完整的修改历史追踪与版本回退，适合游戏本地化和文本 Mod 工作流。

## 核心功能

- **项目化管理** — 每个项目独立目录，包含 SQLite 数据库和格式定义，可随时打开/关闭
- **修正历史记录** — 自动追踪所有文本字段的变更，支持按条目查看历史并回退到任意版本
- **可扩展格式系统** — 通过 YAML 定义文件格式，支持三种模式：
  - 结构化 (JSON / XML / CSV / YAML)
  - 正则提取 (Lua / C# / 任意源码)
  - 二进制 (指针表 + 字符串编码)
- **导入预览** — 导入前可预览提取结果，确认无误再写入数据库
- **搜索与筛选** — 按关键词搜索、按修改状态过滤条目

## 技术栈

| 层级 | 技术 |
|------|------|
| 语言 | Python 3.10+ |
| UI 框架 | PySide6 (Qt for Python) |
| 数据库 | SQLite |
| 格式定义 | YAML (PyYAML) |
| 编码检测 | chardet |

## 文件说明

- `main.py` — 应用入口
- `requirements.txt` — Python 依赖
- `core/models.py` — 数据类 (TextEntry, HistoryRecord, SourceFile)
- `core/database.py` — SQLite CRUD 操作
- `core/history.py` — 变更追踪与回退逻辑
- `core/project.py` — 项目生命周期管理
- `core/format_engine.py` — 格式引擎 (加载定义、导入、导出)
- `formats/base.py` — FormatHandler 抽象基类
- `formats/structured_handler.py` — JSON/XML/CSV/YAML 处理器
- `formats/regex_handler.py` — 正则提取处理器
- `formats/binary_handler.py` — 二进制格式处理器
- `ui/main_window.py` — 主窗口布局与菜单
- `ui/entry_table.py` — 文本条目表格视图
- `ui/editor_panel.py` — 单条目编辑面板
- `ui/history_panel.py` — 历史记录面板
- `ui/import_dialog.py` — 导入对话框 (带预览)
- `ui/export_dialog.py` — 导出对话框
- `ui/format_editor.py` — 格式定义 YAML 编辑器
- `builtin_formats/` — 内置格式定义 (JSON KV, JSON Array, CSV, Lua, C#)

## 运行方式

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
python main.py
```
