# Mini Reader 使用文档

一个小巧、安静、纯粹的本地 TOEFL / IELTS 英文阅读器，运行在 macOS 上。

---

## 目录

- [1. 环境准备](#1-环境准备)
- [2. 安装依赖](#2-安装依赖)
- [3. 运行（开发模式）](#3-运行开发模式)
- [4. 打包（生产构建）](#4-打包生产构建)
- [5. 界面与操作指南](#5-界面与操作指南)
- [6. 添加你自己的文章](#6-添加你自己的文章)
- [7. 数据存放位置](#7-数据存放位置)
- [8. 常见问题](#8-常见问题)
- [9. 项目结构速查](#9-项目结构速查)

---

## 1. 环境准备

| 工具       | 版本要求 | 验证命令             |
| ---------- | -------- | -------------------- |
| macOS      | 11+      | `sw_vers`            |
| Node.js    | 18+      | `node -v` (本机 22)  |
| npm        | 9+       | `npm -v` (本机 10.9) |
| Rust       | 1.75+    | `rustc --version`    |
| Xcode CLT  | 任意     | `xcode-select -p`    |

如果 `rustc` 报错（如 "Missing manifest"）：

```bash
rustup toolchain install stable --force
rustup default stable
```

如果 Xcode 命令行工具未安装：

```bash
xcode-select --install
```

---

## 2. 安装依赖

进入项目根目录：

```bash
cd /Users/weichenguang.kasei/jsproj/fishreader
npm install
```

首次安装：
- npm 依赖会快速完成（约 10 秒）。
- 首次执行 `npm run tauri dev` 或 `cargo check` 时，Rust 端会下载并编译所有 crate（含 `tauri`、`rusqlite` 等），可能耗时 3–10 分钟，**这是一次性的**。

---

## 3. 运行（开发模式）

```bash
npm run tauri dev
```

启动流程：

1. Vite 在 `http://localhost:1420` 启动前端。
2. Tauri 编译 Rust 后端并打开一个 macOS 桌面窗口（约 420 × 620，位于屏幕中央）。
3. 启动时会自动扫描 [`src-tauri/resources/articles/`](src-tauri/resources/articles) 下的 Markdown 文章并写入 SQLite。

热重载：
- 前端文件 (`src/**`) 修改后自动刷新。
- Rust 文件 (`src-tauri/src/**`) 修改后会自动重新编译并重启窗口。

仅校验代码：

```bash
# 前端类型检查
npx tsc -b

# 前端打包（生成 dist/）
npx vite build

# 后端编译检查
cd src-tauri && cargo check
```

---

## 4. 打包（生产构建）

```bash
npm run tauri build
```

产物位置：

```
src-tauri/target/release/bundle/macos/Mini Reader.app   # 可双击运行
src-tauri/target/release/bundle/dmg/Mini Reader_*.dmg   # 分发用 DMG
```

> **图标说明**：项目自带一张占位 PNG ([`src-tauri/icons/icon.png`](src-tauri/icons/icon.png)) 让 Rust 能编译通过。
> 如需正式发布，请用一张 1024×1024 PNG 生成完整图标集：
>
> ```bash
> npx tauri icon path/to/your-icon.png
> ```
>
> 命令会自动生成 `icons/32x32.png`、`128x128.png`、`icon.icns` 等，然后把这些路径补回 [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) 的 `bundle.icon` 数组。

---

## 5. 界面与操作指南

### 5.1 启动行为

- **首次启动**：显示文章列表（默认显示全部 TOEFL + IELTS 示例文章）。
- **再次启动**：自动恢复上次阅读的文章和滚动位置。

### 5.2 文章列表页

```
┌─────────────────────────────┐
│ Mini Reader              ⚙  │  ← 顶部栏（设置入口）
├─────────────────────────────┤
│ [All] [TOEFL] [IELTS]   [★] │  ← 过滤栏
├─────────────────────────────┤
│ 🔍 Search title, source…    │  ← 搜索框
├─────────────────────────────┤
│ Groundwater                ★│
│ TOEFL · TPO 01 · 42%        │  ← 条目（含进度）
├─────────────────────────────┤
│ The Origin of Meteorites    │
│ TOEFL · TPO 02              │
└─────────────────────────────┘
```

操作：

| 操作              | 方法                                |
| ----------------- | ----------------------------------- |
| 按考试类型过滤    | 点击 `All` / `TOEFL` / `IELTS` 芯片 |
| 仅看收藏文章      | 点击右上角 ★                        |
| 搜索              | 在搜索框输入（实时匹配标题/来源/标签）|
| 打开文章          | 点击列表项                          |
| 进入设置          | 点击右上角 ⚙                        |

### 5.3 阅读页

```
┌─────────────────────────────┐
│ ←  TOEFL · TPO 01        ☆ │  ← 返回 / 来源 / 收藏
├─────────────────────────────┤
│ Groundwater                 │
│                             │
│ Groundwater is the word…    │  ← Markdown 渲染
│                             │
│ The water table is…         │
├─────────────────────────────┤
│ 42%                         │  ← 阅读进度
└─────────────────────────────┘
```

操作：

| 操作         | 方法                                          |
| ------------ | --------------------------------------------- |
| 返回列表     | 左上角 ←                                       |
| 收藏 / 取消  | 右上角 ☆ / ★                                  |
| 调整字号     | 设置 ⚙（页面右上角浮动按钮也可进入）          |
| 滚动         | 触摸板 / 鼠标滚轮 / 方向键                    |
| 自动保存进度 | 滚动停下 ~600ms 后自动写入；关窗前也会保存一次 |

### 5.4 设置面板

| 设置项        | 选项                              | 说明                                  |
| ------------- | --------------------------------- | ------------------------------------- |
| Theme         | light / dark / system             | system 跟随 macOS 外观，实时切换       |
| Font size     | 14 – 24 px（A- / A+）             | 实时生效                              |
| Line height   | 1.5 / 1.75 / 2                    | 实时生效                              |
| Always on top | On / Off                          | 窗口始终置顶，适合摸鱼边角阅读        |

所有设置立刻持久化到 SQLite，重启后保留。

---

## 6. 添加你自己的文章

### 6.1 文件格式

每篇文章是一份 Markdown 文件，文件头使用 YAML-like front matter：

```markdown
---
id: my-custom-article-01
exam: TOEFL
source: My Notes
title: A Day in the Library
tags:
  - daily
  - reading
difficulty: easy
---

Paragraph one of the article goes here…

Paragraph two…

Paragraph three…
```

字段说明：

| 字段         | 是否必填 | 说明                                          |
| ------------ | -------- | --------------------------------------------- |
| `id`         | 推荐     | 全局唯一；缺省时用文件名                      |
| `exam`       | 推荐     | `TOEFL` 或 `IELTS`（决定它出现在哪个分类）    |
| `source`     | 可选     | 来源标识，例如 `TPO 01`、`Cambridge IELTS 17` |
| `title`      | 必填     | 显示标题                                      |
| `tags`       | 可选     | YAML 列表，用于搜索匹配                       |
| `difficulty` | 可选     | `easy` / `medium` / `hard`                    |

### 6.2 放置位置

把文件放进：

```
src-tauri/resources/articles/toefl/your-article.md
src-tauri/resources/articles/ielts/your-article.md
```

子目录名不影响分类（分类来自 front matter 的 `exam`），但建议按考试类型组织。

### 6.3 让应用识别

- **开发模式**：保存文件 → Rust 自动重启 → 重启后自动同步到数据库 → 出现在列表。
- **打包后**：这些文件会随应用一起打包到 `.app/Contents/Resources/` 中。

> 当前版本不支持运行时"添加目录"的 UI 导入；如需扩展，请见 [`DESIGN.md`](DESIGN.md) 第 9.1 节的 `import_articles_from_directory` 命令规划。

---

## 7. 数据存放位置

SQLite 数据库位于 macOS 应用数据目录：

```
~/Library/Application Support/com.fishreader.app/reader.db
```

包含四张表：`articles`（文章索引）、`reading_progress`（阅读进度）、`favorites`（收藏）、`settings`（设置 + KV）。

**重置应用数据**：删除该文件即可。下次启动会重新建库并重新扫描内置文章。

```bash
rm "~/Library/Application Support/com.fishreader.app/reader.db"
```

---

## 8. 常见问题

**Q1：首次 `npm run tauri dev` 卡了很久？**
A：Rust 在编译依赖（约 200+ crate），属正常情况。后续启动会走增量编译，秒级。

**Q2：窗口空白 / 文章列表显示 "No articles yet."？**
A：可能是 [`src-tauri/resources/articles/`](src-tauri/resources/articles) 下没有 `.md` 文件，或 front matter 格式错误（缺少 `---` 分隔符）。终端会打印 `[articles] skip ...` 告知跳过原因。

**Q3：进度没保存？**
A：进度保存有 600ms debounce。快速滚动并立刻关窗（在 debounce 触发前）可能丢一次；卸载组件时还有一次兜底保存。

**Q4：切换到深色模式后部分元素不协调？**
A：所有颜色通过 CSS 变量集中定义在 [`src/styles/global.css`](src/styles/global.css)，调整 `[data-theme="dark"]` 块即可。

**Q5：想改默认窗口大小？**
A：编辑 [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) 中 `app.windows[0]` 的 `width` / `height`，重启 dev 即可。

**Q6：如何修改应用名 / Bundle ID？**
A：编辑 [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json) 的 `productName` 和 `identifier`。注意 `identifier` 改动后数据目录路径也会改变，老数据需要手动迁移。

---

## 9. 项目结构速查

```
fishreader/
├── package.json                       # npm 脚本与依赖
├── vite.config.ts                     # Vite 配置
├── tsconfig.json                      # TS 配置
├── index.html                         # 前端入口 HTML
│
├── src/                               # React + TS 前端
│   ├── main.tsx                       # React 入口
│   ├── App.tsx                        # 顶层路由：list / reader / settings
│   ├── components/
│   │   ├── ArticleList.tsx            # 文章列表 + 过滤 + 搜索
│   │   ├── Reader.tsx                 # 阅读页 + 进度保存
│   │   └── SettingsPanel.tsx          # 设置面板
│   ├── services/
│   │   └── api.ts                     # 包装 Tauri invoke 调用
│   ├── types/                         # TS 类型
│   └── styles/global.css              # 全局样式 + CSS 变量主题
│
├── src-tauri/                         # Tauri / Rust 后端
│   ├── Cargo.toml                     # Rust 依赖
│   ├── tauri.conf.json                # Tauri 配置（窗口/资源/打包）
│   ├── build.rs
│   ├── capabilities/default.json      # 权限白名单
│   ├── icons/icon.png                 # 占位图标
│   ├── resources/articles/            # 内置示例文章
│   │   ├── toefl/*.md
│   │   └── ielts/*.md
│   └── src/
│       ├── main.rs                    # 二进制入口
│       ├── lib.rs                     # Tauri builder / setup
│       ├── models.rs                  # 共享数据类型
│       ├── db.rs                      # SQLite 初始化 + migrations
│       ├── articles.rs                # Markdown front matter 解析
│       └── commands.rs                # 所有 #[tauri::command] 实现
│
├── DESIGN.md                          # 原始设计文档
├── README.md                          # 项目简介
└── USAGE.md                           # 本文档
```

---

需要后续添加：全局快捷键呼出、文件夹导入 UI、最近阅读、随机一篇 等功能，可参考 [`DESIGN.md`](DESIGN.md) 第 18 节的后续版本规划。