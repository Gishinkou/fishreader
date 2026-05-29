下面是一版可以直接给 **AI Coding Agent** 使用的落地文档。目标是让 Agent 不跑偏，不把它做成刷题平台，而是做成一个 **Mac 上小巧、纯粹、摸鱼式阅读 TOEFL / IELTS 真题文章的本地阅读器**。

---

# TOEFL / IELTS Mini Reader

# AI Coding 落地开发文档

## 1. 项目定位

开发一个运行在 macOS 上的本地桌面阅读程序，用于阅读 TOEFL / IELTS 考试真题文章。

核心使用场景：

> 用户上班时可以打开一个小窗口，快速阅读一篇英文文章，不做题、不计分、不进入完整考试流程，只专注于阅读英文原文。

本项目不是刷题系统，不是在线教育平台，不是 AI 学习助手。第一版只做一个轻量、安静、纯粹的本地英文阅读器。

---

## 2. 技术栈选择

### 2.1 首选技术栈

```text
Tauri
React
TypeScript
Vite
SQLite
Markdown
CSS / Tailwind CSS
```

### 2.2 技术职责划分

```text
Tauri
  负责桌面壳、窗口管理、macOS 打包、本地文件访问、全局快捷键。

React + TypeScript
  负责文章列表、阅读页面、设置面板、阅读状态展示。

SQLite
  负责文章索引、阅读进度、收藏状态、阅读历史。

Markdown
  负责保存文章正文。

Vite
  负责前端开发和构建。
```

### 2.3 不采用的技术

第一版不使用：

```text
Electron
Next.js
后端服务
登录系统
云同步
复杂用户系统
AI 总结 / 翻译 / 讲解
做题与评分系统
```

原因：

```text
项目目标是轻量本地阅读器。
不需要服务端。
不需要复杂状态同步。
不应该引入过重架构。
```

---

## 3. 产品原则

Agent 开发时必须遵守以下原则。

### 3.1 功能克制

只做阅读，不做题。

允许做：

```text
文章浏览
文章分类
搜索
阅读进度
收藏
字号调整
暗色模式
小窗口
置顶窗口
快捷键呼出
```

不允许做：

```text
选择题
答案解析
错题本
分数统计
学习报告
AI 讲解
AI 翻译
背单词系统
用户登录
云端同步
社交分享
```

### 3.2 窗口小巧

默认窗口不应像一个完整后台系统，而应像一个小工具。

推荐默认尺寸：

```text
宽度：420px
高度：620px
```

允许用户调整窗口大小，但默认体验应是小窗口阅读。

### 3.3 内容优先

阅读界面应尽量干净。

正文区域优先级最高。

不应有复杂导航栏、营销式卡片、仪表盘。

---

## 4. 第一版功能范围

## 4.1 MVP 功能清单

第一版必须实现：

```text
1. macOS 桌面应用启动
2. 文章列表
3. TOEFL / IELTS 分类筛选
4. 文章阅读页面
5. Markdown 正文渲染
6. 字号调整
7. 明暗主题切换
8. 阅读进度记忆
9. 收藏文章
10. 标题搜索
11. 小窗口默认尺寸
```

第一版可以实现但不是必须：

```text
1. 窗口置顶
2. 全局快捷键呼出 / 隐藏
3. 最近阅读列表
4. 阅读进度百分比
5. 文章标签筛选
```

第一版不要实现：

```text
1. 做题
2. 答案
3. 解析
4. 账户系统
5. 网络请求
6. AI 功能
```

---

# 5. 信息架构

## 5.1 页面结构

应用只有三个主要区域：

```text
Home / Article List
  文章列表页

Reader
  阅读页

Settings
  设置页
```

可以采用单页应用结构，不需要复杂路由。

推荐页面布局：

```text
App
  ├── Sidebar
  │     ├── Exam Type Filter
  │     ├── Search Box
  │     └── Article List
  │
  ├── Reader
  │     ├── Article Header
  │     ├── Article Content
  │     └── Reading Progress
  │
  └── Settings Modal / Drawer
        ├── Font Size
        ├── Theme
        ├── Always On Top
        └── Data Import
```

小窗口状态下，可以使用两种模式：

```text
列表模式：
左侧文章列表 + 右侧文章预览

阅读模式：
只显示正文，隐藏列表
```

建议第一版采用简单模式：

```text
顶部工具栏
正文区域
底部阅读进度
```

文章列表可以通过按钮展开。

---

# 6. 数据设计

## 6.1 文章内容格式

文章正文建议使用 Markdown 文件存储。

示例：

```markdown
---
id: toefl-tpo-01-passage-01
exam: TOEFL
source: TPO 01
title: Groundwater
category: Reading
tags:
  - science
  - geology
difficulty: medium
---

Groundwater is the word used to describe water that saturates the ground...

Paragraph 2...

Paragraph 3...
```

### 字段说明

```text
id
  全局唯一文章 ID。

exam
  TOEFL 或 IELTS。

source
  来源，例如 TPO 01、Cambridge IELTS 17 Test 1。

title
  文章标题。

category
  当前固定为 Reading。

tags
  文章标签，例如 science、history、biology、education。

difficulty
  难度，可选 easy / medium / hard。
```

---

## 6.2 本地目录结构

推荐结构：

```text
app-data/
  articles/
    toefl/
      tpo-01-passage-01.md
      tpo-01-passage-02.md
    ielts/
      cambridge-17-test-1-passage-1.md
      cambridge-17-test-1-passage-2.md

  reader.db
```

如果是随应用内置少量示例文章：

```text
src-tauri/
  resources/
    articles/
      sample-toefl.md
      sample-ielts.md
```

---

## 6.3 SQLite 数据表设计

### articles 表

```sql
CREATE TABLE IF NOT EXISTS articles (
  id TEXT PRIMARY KEY,
  exam_type TEXT NOT NULL,
  source TEXT,
  title TEXT NOT NULL,
  category TEXT,
  tags TEXT,
  difficulty TEXT,
  file_path TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
```

说明：

```text
tags 使用 JSON 字符串保存即可。
file_path 指向本地 Markdown 文件。
```

---

### reading_progress 表

```sql
CREATE TABLE IF NOT EXISTS reading_progress (
  article_id TEXT PRIMARY KEY,
  scroll_position REAL NOT NULL DEFAULT 0,
  progress_percent REAL NOT NULL DEFAULT 0,
  last_read_at INTEGER NOT NULL,
  read_count INTEGER NOT NULL DEFAULT 0
);
```

说明：

```text
scroll_position
  保存滚动位置。

progress_percent
  保存阅读百分比。

last_read_at
  最近阅读时间。

read_count
  阅读次数。
```

---

### favorites 表

```sql
CREATE TABLE IF NOT EXISTS favorites (
  article_id TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL
);
```

---

### settings 表

```sql
CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

常见设置：

```text
theme = light / dark / system
fontSize = 16 / 18 / 20 / 22
alwaysOnTop = true / false
lastOpenedArticleId = xxx
```

---

# 7. 目录结构设计

推荐工程结构：

```text
toefl-ielts-mini-reader/
  package.json
  vite.config.ts
  tsconfig.json
  index.html

  src/
    main.tsx
    App.tsx

    components/
      ArticleList.tsx
      ArticleListItem.tsx
      Reader.tsx
      ReaderToolbar.tsx
      SearchBox.tsx
      SettingsPanel.tsx
      EmptyState.tsx

    pages/
      HomePage.tsx
      ReaderPage.tsx

    hooks/
      useArticles.ts
      useReaderSettings.ts
      useReadingProgress.ts

    services/
      articleService.ts
      progressService.ts
      settingsService.ts
      favoriteService.ts

    types/
      article.ts
      settings.ts

    styles/
      global.css
      reader.css

  src-tauri/
    Cargo.toml
    tauri.conf.json
    src/
      main.rs
      commands/
        article_commands.rs
        settings_commands.rs
        window_commands.rs
      db/
        mod.rs
        migrations.rs
```

---

# 8. 前端类型定义

## 8.1 Article 类型

```ts
export type ExamType = "TOEFL" | "IELTS";

export interface Article {
  id: string;
  examType: ExamType;
  source?: string;
  title: string;
  category?: string;
  tags: string[];
  difficulty?: "easy" | "medium" | "hard";
  filePath: string;
  isFavorite?: boolean;
  progressPercent?: number;
  lastReadAt?: number;
}

export interface ArticleContent {
  article: Article;
  content: string;
}
```

---

## 8.2 ReaderSettings 类型

```ts
export interface ReaderSettings {
  theme: "light" | "dark" | "system";
  fontSize: number;
  lineHeight: number;
  fontFamily: "system" | "serif" | "mono";
  alwaysOnTop: boolean;
}
```

---

## 8.3 ReadingProgress 类型

```ts
export interface ReadingProgress {
  articleId: string;
  scrollPosition: number;
  progressPercent: number;
  lastReadAt: number;
  readCount: number;
}
```

---

# 9. Tauri 命令设计

前端不直接操作文件和数据库，而是通过 Tauri commands 调用 Rust 层。

## 9.1 文章相关命令

```rust
#[tauri::command]
async fn list_articles(
    exam_type: Option<String>,
    keyword: Option<String>,
    favorite_only: Option<bool>,
) -> Result<Vec<Article>, String>
```

用途：

```text
获取文章列表。
支持 TOEFL / IELTS 筛选。
支持标题搜索。
支持收藏筛选。
```

---

```rust
#[tauri::command]
async fn get_article_content(article_id: String) -> Result<ArticleContent, String>
```

用途：

```text
读取指定文章 Markdown 内容。
返回文章元信息和正文。
```

---

```rust
#[tauri::command]
async fn import_articles_from_directory(path: String) -> Result<ImportResult, String>
```

用途：

```text
从本地目录导入 Markdown 文章。
第一版可以不做 UI，只保留命令或后续扩展。
```

---

## 9.2 阅读进度相关命令

```rust
#[tauri::command]
async fn save_reading_progress(
    article_id: String,
    scroll_position: f64,
    progress_percent: f64,
) -> Result<(), String>
```

---

```rust
#[tauri::command]
async fn get_reading_progress(article_id: String) -> Result<Option<ReadingProgress>, String>
```

---

## 9.3 收藏相关命令

```rust
#[tauri::command]
async fn toggle_favorite(article_id: String) -> Result<bool, String>
```

返回值：

```text
true 表示收藏后状态。
false 表示取消收藏后状态。
```

---

## 9.4 设置相关命令

```rust
#[tauri::command]
async fn get_settings() -> Result<ReaderSettings, String>
```

```rust
#[tauri::command]
async fn update_settings(settings: ReaderSettings) -> Result<(), String>
```

---

## 9.5 窗口相关命令

```rust
#[tauri::command]
async fn set_always_on_top(enabled: bool) -> Result<(), String>
```

```rust
#[tauri::command]
async fn toggle_main_window() -> Result<(), String>
```

---

# 10. UI 设计规范

## 10.1 默认窗口

```text
宽度：420px
高度：620px
最小宽度：340px
最小高度：420px
```

窗口应适合放在屏幕角落。

---

## 10.2 阅读页布局

推荐结构：

```text
┌─────────────────────────────┐
│ TOEFL · TPO 01        ⚙︎ ☆  │
├─────────────────────────────┤
│ Groundwater                 │
│                             │
│ Paragraph 1...              │
│                             │
│ Paragraph 2...              │
│                             │
│ Paragraph 3...              │
│                             │
├─────────────────────────────┤
│ 42%                 Aa  ☾   │
└─────────────────────────────┘
```

---

## 10.3 阅读样式

正文应舒适阅读：

```css
.reader-content {
  font-size: 17px;
  line-height: 1.75;
  max-width: 720px;
  padding: 20px;
}

.reader-content p {
  margin-bottom: 1em;
}
```

小窗口下不要使用过宽排版。

---

## 10.4 主题

支持三种：

```text
light
dark
system
```

暗色模式要柔和，不要纯黑强对比。

推荐 CSS 变量：

```css
:root {
  --bg: #f8f8f6;
  --text: #202124;
  --muted: #777;
  --border: #e5e5e5;
}

[data-theme="dark"] {
  --bg: #1f1f1f;
  --text: #eeeeee;
  --muted: #aaaaaa;
  --border: #333333;
}
```

---

# 11. 核心交互

## 11.1 打开应用

启动后：

```text
如果有上次阅读文章：
  自动打开上次文章，并恢复阅读位置。

否则：
  显示文章列表。
```

---

## 11.2 选择文章

用户点击文章列表项后：

```text
1. 加载文章内容
2. 进入阅读页
3. 如果有阅读进度，恢复滚动位置
4. 更新 lastOpenedArticleId
```

---

## 11.3 阅读进度保存

触发时机：

```text
滚动时 debounce 保存
窗口关闭前保存
切换文章前保存
```

建议 debounce：

```text
500ms - 1000ms
```

不要每次滚动都立即写 SQLite。

---

## 11.4 字号调整

底部或设置页提供字号调整：

```text
A-
A+
```

字号范围：

```text
14px - 24px
```

---

## 11.5 搜索

搜索只针对：

```text
标题
source
tags
```

第一版不做全文搜索。

---

# 12. Markdown 解析

前端可以使用：

```text
react-markdown
gray-matter
```

或者 Rust 层解析 front matter 后返回正文。

建议：

```text
Rust 层负责读取文件。
Rust 层解析 front matter。
前端负责渲染 Markdown。
```

这样前端拿到的数据更干净。

---

# 13. 开发任务拆分

## Phase 1：工程初始化

目标：

```text
初始化 Tauri + React + TypeScript 项目。
应用能在 macOS 上启动。
默认窗口尺寸符合要求。
```

任务：

```text
1. 创建 Tauri 项目
2. 接入 React + TypeScript
3. 配置基础 CSS
4. 设置默认窗口尺寸
5. 实现主页面占位 UI
```

验收标准：

```text
npm run tauri dev 可以启动应用。
窗口尺寸约为 420x620。
页面显示应用标题和空状态。
```

---

## Phase 2：文章数据模型与本地示例文章

目标：

```text
应用内置几篇示例文章，可以显示文章列表。
```

任务：

```text
1. 设计 Article 类型
2. 准备 sample TOEFL / IELTS markdown
3. Rust 层读取 resources/articles
4. 解析 front matter
5. 返回文章列表
```

验收标准：

```text
页面能显示至少 3 篇文章。
文章包含 exam、source、title。
可以根据 TOEFL / IELTS 过滤。
```

---

## Phase 3：阅读页

目标：

```text
点击文章后进入阅读页并渲染正文。
```

任务：

```text
1. 实现 ArticleList 组件
2. 实现 Reader 组件
3. 实现 get_article_content 命令
4. 使用 react-markdown 渲染正文
5. 实现返回列表功能
```

验收标准：

```text
点击文章能打开正文。
正文段落排版清晰。
小窗口内阅读体验良好。
```

---

## Phase 4：SQLite 存储

目标：

```text
保存阅读进度、收藏、设置。
```

任务：

```text
1. 初始化 SQLite
2. 创建 migrations
3. 实现 reading_progress 表
4. 实现 favorites 表
5. 实现 settings 表
6. 提供 Tauri commands
```

验收标准：

```text
关闭应用后重新打开，仍能保留设置和阅读进度。
```

---

## Phase 5：阅读进度恢复

目标：

```text
文章滚动位置可以自动保存和恢复。
```

任务：

```text
1. 监听 Reader 滚动
2. 计算 progress_percent
3. debounce 调用 save_reading_progress
4. 打开文章时恢复 scroll_position
5. 底部显示阅读百分比
```

验收标准：

```text
读到一半关闭应用，重新打开后能回到原位置。
底部能显示当前阅读百分比。
```

---

## Phase 6：设置面板

目标：

```text
用户可以调整字号、主题、置顶。
```

任务：

```text
1. 实现 SettingsPanel
2. 实现 fontSize 设置
3. 实现 theme 设置
4. 实现 alwaysOnTop 设置
5. 设置持久化
```

验收标准：

```text
字号调整即时生效。
主题切换即时生效。
设置重启后仍然有效。
```

---

## Phase 7：搜索与收藏

目标：

```text
支持标题搜索和收藏文章。
```

任务：

```text
1. 实现 SearchBox
2. 实现 toggle_favorite
3. 文章列表显示收藏状态
4. 支持 favorite_only 筛选
```

验收标准：

```text
可以搜索文章标题。
可以收藏 / 取消收藏。
可以只看收藏文章。
```

---

# 14. Agent 开发约束

给 AI Coding Agent 的核心约束如下。

## 14.1 不要过度设计

不要引入：

```text
Redux
复杂路由系统
服务端 API
用户认证
云同步
微服务
Docker
GraphQL
```

除非用户明确要求。

---

## 14.2 优先完成可运行 MVP

每个阶段都要保证应用可运行。

不要一次性生成大量无法运行的代码。

开发顺序：

```text
能启动
能显示文章列表
能打开文章
能保存进度
能调整设置
```

---

## 14.3 保持 UI 极简

UI 风格：

```text
安静
小巧
低干扰
适合办公室角落阅读
```

避免：

```text
大面积彩色卡片
复杂 Dashboard
教育平台风格
强营销感
```

---

## 14.4 错误处理

基本要求：

```text
文章读取失败时显示错误提示。
数据库初始化失败时给出清晰错误。
Markdown 解析失败时跳过该文章并记录错误。
设置读取失败时使用默认设置。
```

---

# 15. 推荐默认配置

```ts
export const DEFAULT_READER_SETTINGS: ReaderSettings = {
  theme: "system",
  fontSize: 17,
  lineHeight: 1.75,
  fontFamily: "system",
  alwaysOnTop: false,
};
```

默认窗口：

```json
{
  "width": 420,
  "height": 620,
  "minWidth": 340,
  "minHeight": 420,
  "resizable": true,
  "title": "Mini Reader"
}
```

---

# 16. 文章导入策略

第一版建议不要做复杂导入。

可以采用两种方式：

## 16.1 内置文章

把 Markdown 文件放在应用资源目录中。

优点：

```text
实现简单
适合 MVP
无需文件选择器
```

缺点：

```text
新增文章需要重新打包或手动放目录
```

---

## 16.2 本地目录导入

后续版本支持选择一个目录。

目录格式：

```text
my-articles/
  toefl/
    tpo-01-passage-01.md
  ielts/
    cambridge-17-test-1-passage-1.md
```

导入逻辑：

```text
1. 用户选择目录
2. 扫描 .md 文件
3. 解析 front matter
4. 写入 articles 表
5. file_path 指向原始文件
```

---

# 17. 测试要求

## 17.1 前端测试点

```text
ArticleList 可以正确渲染文章列表。
搜索关键词可以过滤标题。
TOEFL / IELTS 筛选有效。
Reader 可以渲染 Markdown。
字号调整后正文样式变化。
收藏按钮状态正确。
```

---

## 17.2 Rust / Tauri 测试点

```text
可以读取 articles 目录。
可以解析 Markdown front matter。
SQLite 表可以初始化。
阅读进度可以保存和读取。
设置可以保存和读取。
收藏状态可以切换。
```

---

## 17.3 手工验收用例

### 用例 1：启动应用

```text
打开应用
看到文章列表
窗口尺寸小巧
```

通过条件：

```text
应用正常启动。
没有空白页。
窗口不是全屏大应用。
```

---

### 用例 2：阅读文章

```text
点击一篇 TOEFL 文章
进入阅读页
滚动阅读
```

通过条件：

```text
正文正常显示。
段落排版舒适。
滚动不卡顿。
```

---

### 用例 3：恢复进度

```text
打开文章
滚动到 50%
关闭应用
重新打开应用
```

通过条件：

```text
自动回到上次文章。
滚动位置接近关闭前位置。
```

---

### 用例 4：调整字号

```text
打开文章
点击 A+
点击 A-
```

通过条件：

```text
正文字号即时变化。
设置重启后仍保留。
```

---

### 用例 5：收藏文章

```text
打开文章
点击收藏
回到列表
开启只看收藏
```

通过条件：

```text
该文章出现在收藏列表。
再次点击可以取消收藏。
```

---

# 18. 后续版本规划

## V1.1

```text
全局快捷键呼出 / 隐藏
窗口置顶
最近阅读
标签筛选
导入文章目录
```

## V1.2

```text
全文搜索
阅读时间统计
段落折叠
随机文章
每日一篇
```

## V2.0

可以考虑但不优先：

```text
单词高亮
生词收藏
本地词典
简单划词翻译
```

仍然不要做：

```text
完整刷题系统
复杂考试模拟
在线课程平台
```

---

# 19. 给 AI Coding Agent 的总提示词

可以把下面这段直接给 Agent：

```text
你要开发一个 macOS 本地桌面应用，名称暂定为 TOEFL IELTS Mini Reader。

项目目标是做一个小巧、纯粹、适合上班间隙阅读 TOEFL / IELTS 真题阅读文章的本地阅读器。它不是刷题系统，不做题、不评分、不展示答案解析，只负责文章阅读。

技术栈使用 Tauri + React + TypeScript + Vite + SQLite + Markdown。

请按 MVP 优先原则开发，避免过度设计。不要引入服务端、登录、云同步、Redux、复杂路由、AI 功能、题目系统。

第一版功能包括：
1. macOS 桌面应用启动；
2. 默认小窗口，约 420x620；
3. 显示 TOEFL / IELTS 文章列表；
4. 支持标题搜索；
5. 支持 TOEFL / IELTS 分类筛选；
6. 点击文章进入阅读页；
7. Markdown 正文渲染；
8. 支持字号调整；
9. 支持 light / dark / system 主题；
10. 支持收藏文章；
11. 自动保存并恢复阅读进度；
12. 设置持久化到 SQLite。

UI 风格要极简、安静、小巧，类似 Mac 便签 + Kindle 阅读器，不要做成后台管理系统或在线教育平台。

请先完成一个可以运行的 MVP：
阶段一：初始化 Tauri + React + TypeScript 项目；
阶段二：加入几篇本地 Markdown 示例文章；
阶段三：实现文章列表和阅读页；
阶段四：加入 SQLite 保存阅读进度和设置；
阶段五：实现收藏、搜索、主题、字号。

每个阶段都必须保证应用可以运行。
请保持代码结构清晰，使用 TypeScript 类型定义，Rust 侧通过 Tauri commands 暴露本地文件和 SQLite 能力。
```

---

# 20. 更强约束版 Agent 提示词

如果你担心 Agent 跑偏，可以使用下面这版：

```text
严格限制：
不要实现做题功能。
不要实现答案解析。
不要实现错题本。
不要实现登录。
不要实现云同步。
不要实现 AI 翻译、AI 总结、AI 讲解。
不要引入后端服务。
不要引入 Redux。
不要把 UI 做成后台管理系统。
不要做复杂 Dashboard。

你只需要实现一个本地英文文章阅读器。

任何新增功能都必须服务于“更舒服地阅读文章”这个目标。
```

---

# 21. 最小 MVP 骨架

如果想进一步压缩第一版，可以只让 Agent 做这个：

```text
1. Tauri + React 应用
2. 内置 3 篇 Markdown 文章
3. 左侧文章列表
4. 右侧阅读正文
5. 字号调整
6. 暗色模式
7. SQLite 保存最后阅读文章和滚动位置
```

这个版本完成后，再逐步加收藏、搜索、导入目录、置顶窗口。

---

我建议你实际开发时，把任务切成两个 Agent 迭代：

```text
第一轮：
只做能运行的阅读器 MVP。

第二轮：
再加 SQLite 持久化、设置、收藏、搜索、窗口置顶。
```

这样最不容易被 AI Coding Agent 写成一个庞大但跑不起来的半成品。
