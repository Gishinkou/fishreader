# Mini Reader (fishreader)

A tiny, quiet, local-first desktop reader for TOEFL / IELTS reading passages.
No quizzes, no scoring, no AI — just the article.

Built with Tauri 2 + React + TypeScript + Vite + SQLite.

## Features

- TOEFL / IELTS article list with filters and title search
- Markdown article rendering
- Auto-saved reading progress (per article)
- Restore last opened article on launch
- Favorites
- Font size and line height controls
- Light / dark / system theme
- Always-on-top toggle
- Small default window (420 × 620)

## Project layout

```
src/                React + TS frontend
src-tauri/src/      Rust backend (commands, SQLite, markdown front matter)
src-tauri/resources/articles/  Built-in sample articles (Markdown)
```

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

Sample articles live in `src-tauri/resources/articles/{toefl,ielts}/*.md`.
Each file uses YAML-style front matter (`id`, `exam`, `source`, `title`, `tags`, …).