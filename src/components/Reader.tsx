import { useEffect, useRef, useState, useCallback } from "react";
import ReactMarkdown from "react-markdown";
import type { ArticleContent } from "../types/article";
import {
  addNote,
  getArticleContent,
  getReadingProgress,
  saveReadingProgress,
  toggleFavorite,
} from "../services/api";
import SelectionPopover from "./SelectionPopover";

interface Props {
  articleId: string;
  fontSize: number;
  lineHeight: number;
  onBack: () => void;
  onFavoriteChanged?: () => void;
}

export default function Reader({
  articleId,
  fontSize,
  lineHeight,
  onBack,
  onFavoriteChanged,
}: Props) {
  const [data, setData] = useState<ArticleContent | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [progress, setProgress] = useState(0);
  const [isFav, setIsFav] = useState(false);
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const saveTimerRef = useRef<number | null>(null);
  const restoredRef = useRef(false);
  const [popover, setPopover] = useState<{
    x: number;
    y: number;
    text: string;
    context: string;
  } | null>(null);
  const [toast, setToast] = useState<string | null>(null);
  const toastTimerRef = useRef<number | null>(null);

  useEffect(() => {
    let cancelled = false;
    restoredRef.current = false;
    setData(null);
    setError(null);
    setProgress(0);

    (async () => {
      try {
        const ac = await getArticleContent(articleId);
        if (cancelled) return;
        setData(ac);
        setIsFav(ac.article.isFavorite);

        const prog = await getReadingProgress(articleId);
        if (cancelled) return;

        // Wait a frame for markdown layout
        requestAnimationFrame(() => {
          if (cancelled) return;
          const el = scrollRef.current;
          if (el && prog) {
            el.scrollTop = prog.scrollPosition || 0;
            setProgress(prog.progressPercent || 0);
          }
          restoredRef.current = true;
        });
      } catch (e) {
        if (!cancelled) setError(String(e));
      }
    })();

    return () => {
      cancelled = true;
      if (saveTimerRef.current) {
        window.clearTimeout(saveTimerRef.current);
      }
    };
  }, [articleId]);

  const handleScroll = useCallback(() => {
    const el = scrollRef.current;
    if (!el) return;
    const max = el.scrollHeight - el.clientHeight;
    const pct = max > 0 ? (el.scrollTop / max) * 100 : 0;
    setProgress(pct);
    if (!restoredRef.current) return;

    if (saveTimerRef.current) {
      window.clearTimeout(saveTimerRef.current);
    }
    const scrollTop = el.scrollTop;
    saveTimerRef.current = window.setTimeout(() => {
      saveReadingProgress(articleId, scrollTop, pct).catch(() => {
        /* ignore */
      });
    }, 600);
  }, [articleId]);

  // Save on unmount as well.
  useEffect(() => {
    return () => {
      const el = scrollRef.current;
      if (el && restoredRef.current) {
        const max = el.scrollHeight - el.clientHeight;
        const pct = max > 0 ? (el.scrollTop / max) * 100 : 0;
        saveReadingProgress(articleId, el.scrollTop, pct).catch(() => {});
      }
    };
  }, [articleId]);

  // Selection -> floating popover.
  const updatePopoverFromSelection = useCallback(() => {
    const sel = window.getSelection();
    if (!sel || sel.isCollapsed || sel.rangeCount === 0) {
      setPopover(null);
      return;
    }
    const text = sel.toString().trim();
    if (!text) {
      setPopover(null);
      return;
    }
    // Only react to selections inside the reader-scroll area.
    const scrollEl = scrollRef.current;
    const anchor = sel.anchorNode;
    if (!scrollEl || !anchor || !scrollEl.contains(anchor)) {
      return;
    }
    const range = sel.getRangeAt(0);
    const rect = range.getBoundingClientRect();
    if (rect.width === 0 && rect.height === 0) {
      setPopover(null);
      return;
    }
    // Try to capture surrounding sentence/paragraph as context.
    let context = "";
    const node = range.startContainer;
    const blockEl =
      (node.nodeType === Node.ELEMENT_NODE
        ? (node as HTMLElement)
        : node.parentElement)?.closest("p, li, h1, h2, h3, blockquote") ?? null;
    if (blockEl) context = blockEl.textContent?.trim() ?? "";

    setPopover({
      x: rect.left + rect.width / 2,
      y: rect.top - 8,
      text,
      context,
    });
  }, []);

  useEffect(() => {
    const onMouseUp = () => {
      // Defer so the selection has settled.
      window.setTimeout(updatePopoverFromSelection, 0);
    };
    const onSelectionChange = () => {
      const sel = window.getSelection();
      if (!sel || sel.isCollapsed) setPopover(null);
    };
    document.addEventListener("mouseup", onMouseUp);
    document.addEventListener("selectionchange", onSelectionChange);
    return () => {
      document.removeEventListener("mouseup", onMouseUp);
      document.removeEventListener("selectionchange", onSelectionChange);
    };
  }, [updatePopoverFromSelection]);

  const showToast = useCallback((msg: string) => {
    setToast(msg);
    if (toastTimerRef.current) window.clearTimeout(toastTimerRef.current);
    toastTimerRef.current = window.setTimeout(() => setToast(null), 1400);
  }, []);

  const handleCopy = useCallback(() => {
    if (!popover) return;
    navigator.clipboard
      .writeText(popover.text)
      .then(() => showToast("Copied"))
      .catch(() => showToast("Copy failed"));
  }, [popover, showToast]);

  const handleSave = useCallback(() => {
    if (!popover) return;
    addNote({
      articleId,
      text: popover.text,
      context: popover.context || null,
    })
      .then((n) =>
        showToast(n.kind === "word" ? "Saved (word)" : "Saved (phrase)"),
      )
      .catch(() => showToast("Save failed"));
  }, [popover, articleId, showToast]);

  const onToggleFav = async () => {
    try {
      const newState = await toggleFavorite(articleId);
      setIsFav(newState);
      onFavoriteChanged?.();
    } catch {
      /* ignore */
    }
  };

  return (
    <div className="reader">
      <div className="toolbar">
        <button className="icon-btn" onClick={onBack} title="Back">
          ←
        </button>
        <div className="crumb">
          {data
            ? `${data.article.examType}${
                data.article.source ? " · " + data.article.source : ""
              }`
            : ""}
        </div>
        <button
          className="icon-btn"
          onClick={onToggleFav}
          title={isFav ? "Unfavorite" : "Favorite"}
        >
          <span className={isFav ? "fav-star" : ""}>{isFav ? "★" : "☆"}</span>
        </button>
      </div>
      <div
        className="reader-scroll"
        ref={scrollRef}
        onScroll={handleScroll}
        style={{ fontSize: `${fontSize}px`, lineHeight }}
      >
        {error && <div className="empty">Failed to load: {error}</div>}
        {!error && !data && <div className="empty">Loading…</div>}
        {data && (
          <div className="reader-content">
            <h1>{data.article.title}</h1>
            <ReactMarkdown>{data.content}</ReactMarkdown>
          </div>
        )}
      </div>
      <div className="reader-footer">
        <div className="pct">{Math.round(progress)}%</div>
      </div>
      {popover && (
        <SelectionPopover
          x={popover.x}
          y={popover.y}
          onCopy={handleCopy}
          onSave={handleSave}
          onDismiss={() => setPopover(null)}
        />
      )}
      {toast && <div className="toast">{toast}</div>}
    </div>
  );
}