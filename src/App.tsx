import { useEffect, useState, useCallback } from "react";
import ArticleList from "./components/ArticleList";
import Reader from "./components/Reader";
import SettingsPanel from "./components/SettingsPanel";
import {
  getSettings,
  updateSettings,
  setKv,
  getKv,
  setAlwaysOnTop,
} from "./services/api";
import type { Article } from "./types/article";
import {
  DEFAULT_READER_SETTINGS,
  type ReaderSettings,
} from "./types/settings";

type View = "list" | "reader";

const LAST_OPENED_KEY = "lastOpenedArticleId";

export default function App() {
  const [view, setView] = useState<View>("list");
  const [openedArticleId, setOpenedArticleId] = useState<string | null>(null);
  const [settings, setSettings] = useState<ReaderSettings>(
    DEFAULT_READER_SETTINGS,
  );
  const [settingsLoaded, setSettingsLoaded] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);

  // Load settings + last opened article on startup.
  useEffect(() => {
    (async () => {
      try {
        const s = await getSettings();
        setSettings(s);
      } catch {
        /* keep defaults */
      } finally {
        setSettingsLoaded(true);
      }
      try {
        const last = await getKv(LAST_OPENED_KEY);
        if (last) {
          setOpenedArticleId(last);
          setView("reader");
        }
      } catch {
        /* ignore */
      }
    })();
  }, []);

  // Apply theme to document.
  useEffect(() => {
    const t =
      settings.theme === "system"
        ? window.matchMedia("(prefers-color-scheme: dark)").matches
          ? "dark"
          : "light"
        : settings.theme;
    document.documentElement.setAttribute("data-theme", t);
  }, [settings.theme]);

  // Listen to system theme changes when in system mode.
  useEffect(() => {
    if (settings.theme !== "system") return;
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = () => {
      document.documentElement.setAttribute(
        "data-theme",
        mq.matches ? "dark" : "light",
      );
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, [settings.theme]);

  // Persist settings whenever they change (after initial load).
  useEffect(() => {
    if (!settingsLoaded) return;
    updateSettings(settings).catch(() => {});
    setAlwaysOnTop(settings.alwaysOnTop).catch(() => {});
  }, [settings, settingsLoaded]);

  const openArticle = useCallback((a: Article) => {
    setOpenedArticleId(a.id);
    setView("reader");
    setKv(LAST_OPENED_KEY, a.id).catch(() => {});
  }, []);

  const back = useCallback(() => {
    setView("list");
    setRefreshKey((k) => k + 1);
  }, []);

  return (
    <div className="app">
      {view === "list" && (
        <>
          <div className="toolbar">
            <div className="crumb">Mini Reader</div>
            <button
              className="icon-btn"
              onClick={() => setShowSettings(true)}
              title="Settings"
            >
              ⚙
            </button>
          </div>
          <div className="content">
            <ArticleList onOpen={openArticle} refreshKey={refreshKey} />
          </div>
        </>
      )}

      {view === "reader" && openedArticleId && (
        <>
          <div className="content">
            <Reader
              articleId={openedArticleId}
              fontSize={settings.fontSize}
              lineHeight={settings.lineHeight}
              onBack={back}
              onFavoriteChanged={() => setRefreshKey((k) => k + 1)}
            />
          </div>
          <div
            style={{
              position: "absolute",
              top: 6,
              right: 6,
              zIndex: 5,
            }}
          >
            <button
              className="icon-btn"
              onClick={() => setShowSettings(true)}
              title="Settings"
              style={{ background: "transparent" }}
            >
              ⚙
            </button>
          </div>
        </>
      )}

      {showSettings && (
        <SettingsPanel
          settings={settings}
          onChange={setSettings}
          onClose={() => setShowSettings(false)}
        />
      )}
    </div>
  );
}