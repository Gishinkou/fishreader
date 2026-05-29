export type ExamType = "TOEFL" | "IELTS";

export interface Article {
  id: string;
  examType: string;
  source?: string | null;
  title: string;
  category?: string | null;
  tags: string[];
  difficulty?: "easy" | "medium" | "hard" | string | null;
  filePath: string;
  isFavorite: boolean;
  progressPercent: number;
  lastReadAt?: number | null;
}

export interface ArticleContent {
  article: Article;
  content: string;
}

export interface ReadingProgress {
  articleId: string;
  scrollPosition: number;
  progressPercent: number;
  lastReadAt: number;
  readCount: number;
}