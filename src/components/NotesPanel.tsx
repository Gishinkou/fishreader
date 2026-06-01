import { useEffect, useState, useCallback } from "react";
import type { Note } from "../types/note";
import { deleteNote, listNotes } from "../services/api";

interface Props {
  onClose: () => void;
}

type Filter = "all" | "word" | "phrase";

export default function NotesPanel({ onClose }: Props) {
  const [filter, setFilter] = useState<Filter>("all");
  const [keyword, setKeyword] = useState("");
  const [items, setItems] = useState<Note[]>([]);
  const [loading, setLoading] = useState(false);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      const list = await listNotes({
        kind: filter === "all" ? null : filter,
        keyword: keyword.trim() || null,
      });
      setItems(list);
    } finally {
      setLoading(false);
    }
  }, [filter, keyword]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const handleDelete = async (id: number) => {
    try {
      await deleteNote(id);
      setItems((arr) => arr.filter((n) => n.id !== id));
    } catch {
      /* ignore */
    }
  };

const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text).catch(() => {});
  };

  return (
    <div className="settings-panel">
      <div className="toolbar" style={{ marginLeft: -16, marginRight: -16 }}>
        <button className="icon-btn" onClick={onClose} title="Back">
          ←
        </button>
        <div className="crumb">Notebook</div>
      </div>

      <div className="list-filters" style={{ paddingLeft: 0, paddingRight: 0 }}>
        <button
          className={`chip ${filter === "all" ? "active" : ""}`}
          onClick={() => setFilter("all")}
        >
          All
        </button>
        <button
          className={`chip ${filter === "word" ? "active" : ""}`}
          onClick={() => setFilter("word")}
        >
          Words
        </button>
        <button
          className={`chip ${filter === "phrase" ? "active" : ""}`}
          onClick={() => setFilter("phrase")}
        >
          Phrases
        </button>
        <span style={{ flex: 1 }} />
        <span style={{ fontSize: 11, color: "var(--muted)" }}>
          {items.length}
        </span>
      </div>

      <div className="search-box" style={{ paddingLeft: 0, paddingRight: 0 }}>
        <input
          type="search"
          placeholder="Search text or context…"
          value={keyword}
          onChange={(e) => setKeyword(e.target.value)}
        />
      </div>

      <div className="notes-list">
        {loading && <div className="empty">Loading…</div>}
        {!loading && items.length === 0 && (
          <div className="empty">
            No notes yet. Select text in an article and tap Save.
          </div>
        )}
        {!loading &&
          items.map((n) => (
            <div className="note-item" key={n.id}>
              <div className="note-text">{n.text}</div>
              {n.context && n.context !== n.text && (
                <div className="note-context">{n.context}</div>
              )}
              <div className="note-meta">
                <span className={`tag tag-${n.kind}`}>{n.kind}</span>
                {n.articleTitle && (
                  <span className="note-source">· {n.articleTitle}</span>
                )}
                <span style={{ flex: 1 }} />
                <button
                  className="link-btn"
                  onClick={() => handleCopy(n.text)}
                >
                  Copy
                </button>
                <button
                  className="link-btn danger"
                  onClick={() => handleDelete(n.id)}
                >
                  Delete
                </button>
              </div>
            </div>
          ))}
      </div>
    </div>
  );
}