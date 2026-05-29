import { useEffect, useState } from "react";
import type { Article } from "../types/article";
import { listArticles } from "../services/api";

interface Props {
  onOpen: (a: Article) => void;
  refreshKey?: number;
}

type Filter = "ALL" | "TOEFL" | "IELTS";

export default function ArticleList({ onOpen, refreshKey }: Props) {
  const [filter, setFilter] = useState<Filter>("ALL");
  const [favoriteOnly, setFavoriteOnly] = useState(false);
  const [keyword, setKeyword] = useState("");
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    listArticles({
      examType: filter === "ALL" ? null : filter,
      keyword: keyword.trim() || null,
      favoriteOnly,
    })
      .then((data) => {
        if (!cancelled) setArticles(data);
      })
      .catch((e) => {
        if (!cancelled) setError(String(e));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [filter, keyword, favoriteOnly, refreshKey]);

  return (
    <div className="list-pane">
      <div className="list-filters">
        {(["ALL", "TOEFL", "IELTS"] as Filter[]).map((f) => (
          <button
            key={f}
            className={`chip ${filter === f ? "active" : ""}`}
            onClick={() => setFilter(f)}
          >
            {f === "ALL" ? "All" : f}
          </button>
        ))}
        <div style={{ flex: 1 }} />
        <button
          className={`chip ${favoriteOnly ? "active" : ""}`}
          onClick={() => setFavoriteOnly((v) => !v)}
          title="Favorites only"
        >
          ★
        </button>
      </div>
      <div className="search-box">
        <input
          type="search"
          placeholder="Search title, source, tag…"
          value={keyword}
          onChange={(e) => setKeyword(e.target.value)}
        />
      </div>
      <div className="article-list">
        {loading && <div className="empty">Loading…</div>}
        {error && <div className="empty">Error: {error}</div>}
        {!loading && !error && articles.length === 0 && (
          <div className="empty">No articles yet.</div>
        )}
        {!loading &&
          articles.map((a) => (
            <div
              key={a.id}
              className="article-item"
              onClick={() => onOpen(a)}
            >
              <div className="row">
                <div className="title">{a.title}</div>
                {a.isFavorite && <span className="fav-star">★</span>}
              </div>
              <div className="meta">
                {a.examType}
                {a.source ? ` · ${a.source}` : ""}
                {a.progressPercent > 0
                  ? ` · ${Math.round(a.progressPercent)}%`
                  : ""}
              </div>
            </div>
          ))}
      </div>
    </div>
  );
}