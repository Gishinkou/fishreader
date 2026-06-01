export type NoteKind = "word" | "phrase";

export interface Note {
  id: number;
  articleId?: string | null;
  kind: NoteKind | string;
  text: string;
  context?: string | null;
  createdAt: number;
  articleTitle?: string | null;
}