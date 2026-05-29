import { invoke } from "@tauri-apps/api/core";
import type { Article, ArticleContent, ReadingProgress } from "../types/article";
import type { ReaderSettings } from "../types/settings";

export async function listArticles(opts: {
  examType?: string | null;
  keyword?: string | null;
  favoriteOnly?: boolean;
}): Promise<Article[]> {
  return await invoke<Article[]>("list_articles", {
    examType: opts.examType ?? null,
    keyword: opts.keyword ?? null,
    favoriteOnly: opts.favoriteOnly ?? false,
  });
}

export async function getArticleContent(articleId: string): Promise<ArticleContent> {
  return await invoke<ArticleContent>("get_article_content", { articleId });
}

export async function saveReadingProgress(
  articleId: string,
  scrollPosition: number,
  progressPercent: number,
): Promise<void> {
  await invoke("save_reading_progress", {
    articleId,
    scrollPosition,
    progressPercent,
  });
}

export async function getReadingProgress(
  articleId: string,
): Promise<ReadingProgress | null> {
  return await invoke<ReadingProgress | null>("get_reading_progress", { articleId });
}

export async function toggleFavorite(articleId: string): Promise<boolean> {
  return await invoke<boolean>("toggle_favorite", { articleId });
}

export async function getSettings(): Promise<ReaderSettings> {
  return await invoke<ReaderSettings>("get_settings");
}

export async function updateSettings(settings: ReaderSettings): Promise<void> {
  await invoke("update_settings", { settings });
}

export async function setKv(key: string, value: string): Promise<void> {
  await invoke("set_kv", { key, value });
}

export async function getKv(key: string): Promise<string | null> {
  return await invoke<string | null>("get_kv", { key });
}

export async function setAlwaysOnTop(enabled: boolean): Promise<void> {
  await invoke("set_always_on_top", { enabled });
}