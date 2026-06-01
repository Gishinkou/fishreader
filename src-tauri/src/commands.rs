use crate::articles;
use crate::db::{now_ts, DbState};
use crate::models::{Article, ArticleContent, Note, ReaderSettings, ReadingProgress};
use rusqlite::{params, OptionalExtension};
use std::path::PathBuf;
use tauri::State;

fn map_err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

fn sync_article_row(
    conn: &rusqlite::Connection,
    a: &Article,
) -> Result<(), rusqlite::Error> {
    let tags = serde_json::to_string(&a.tags).unwrap_or_else(|_| "[]".to_string());
    let now = now_ts();
    conn.execute(
        "INSERT INTO articles (id, exam_type, source, title, category, tags, difficulty, file_path, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
         ON CONFLICT(id) DO UPDATE SET
           exam_type=excluded.exam_type,
           source=excluded.source,
           title=excluded.title,
           category=excluded.category,
           tags=excluded.tags,
           difficulty=excluded.difficulty,
           file_path=excluded.file_path,
           updated_at=excluded.updated_at",
        params![
            a.id,
            a.exam_type,
            a.source,
            a.title,
            a.category,
            tags,
            a.difficulty,
            a.file_path,
            now,
        ],
    )?;
    Ok(())
}

/// Scan built-in articles directory (resources) and sync into DB. Called at startup.
pub fn sync_builtin_articles(
    conn: &rusqlite::Connection,
    resource_dir: &PathBuf,
) {
    let articles_dir = resource_dir.join("articles");
    let scanned = articles::scan_articles_dir(&articles_dir);
    for (article, _) in scanned {
        if let Err(e) = sync_article_row(conn, &article) {
            eprintln!("[db] sync article failed: {}", e);
        }
    }
}

#[tauri::command]
pub fn list_articles(
    exam_type: Option<String>,
    keyword: Option<String>,
    favorite_only: Option<bool>,
    state: State<'_, DbState>,
) -> Result<Vec<Article>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut sql = String::from(
        "SELECT a.id, a.exam_type, a.source, a.title, a.category, a.tags, a.difficulty, a.file_path,
                CASE WHEN f.article_id IS NULL THEN 0 ELSE 1 END as is_fav,
                COALESCE(rp.progress_percent, 0) as progress,
                rp.last_read_at
         FROM articles a
         LEFT JOIN favorites f ON f.article_id = a.id
         LEFT JOIN reading_progress rp ON rp.article_id = a.id
         WHERE 1=1",
    );
    let mut args: Vec<String> = Vec::new();
    if let Some(et) = exam_type.filter(|s| !s.is_empty()) {
        sql.push_str(" AND a.exam_type = ?");
        args.push(et);
    }
    if let Some(kw) = keyword.filter(|s| !s.trim().is_empty()) {
        sql.push_str(" AND (a.title LIKE ? OR a.source LIKE ? OR a.tags LIKE ?)");
        let pat = format!("%{}%", kw);
        args.push(pat.clone());
        args.push(pat.clone());
        args.push(pat);
    }
    if favorite_only.unwrap_or(false) {
        sql.push_str(" AND f.article_id IS NOT NULL");
    }
    sql.push_str(" ORDER BY a.exam_type, a.source, a.title");

    let mut stmt = conn.prepare(&sql).map_err(map_err)?;
    let params_dyn: Vec<&dyn rusqlite::ToSql> =
        args.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
    let rows = stmt
        .query_map(params_dyn.as_slice(), |row| {
            let tags_json: String = row.get(5).unwrap_or_else(|_| "[]".to_string());
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            Ok(Article {
                id: row.get(0)?,
                exam_type: row.get(1)?,
                source: row.get(2)?,
                title: row.get(3)?,
                category: row.get(4)?,
                tags,
                difficulty: row.get(6)?,
                file_path: row.get(7)?,
                is_favorite: row.get::<_, i64>(8)? != 0,
                progress_percent: row.get::<_, f64>(9).unwrap_or(0.0),
                last_read_at: row.get(10).ok(),
            })
        })
        .map_err(map_err)?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(map_err)?);
    }
    Ok(out)
}

#[tauri::command]
pub fn get_article_content(
    article_id: String,
    state: State<'_, DbState>,
) -> Result<ArticleContent, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let file_path: String = conn
        .query_row(
            "SELECT file_path FROM articles WHERE id = ?1",
            params![article_id],
            |row| row.get(0),
        )
        .map_err(map_err)?;

    let path = std::path::Path::new(&file_path);
    let (article, body) = articles::parse_markdown_file(path).map_err(map_err)?;

    let is_fav: i64 = conn
        .query_row(
            "SELECT 1 FROM favorites WHERE article_id = ?1",
            params![article.id],
            |row| row.get(0),
        )
        .optional()
        .map_err(map_err)?
        .unwrap_or(0);

    let mut article = article;
    article.is_favorite = is_fav != 0;

    Ok(ArticleContent {
        article,
        content: body,
    })
}

#[tauri::command]
pub fn save_reading_progress(
    article_id: String,
    scroll_position: f64,
    progress_percent: f64,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = now_ts();
    conn.execute(
        "INSERT INTO reading_progress (article_id, scroll_position, progress_percent, last_read_at, read_count)
         VALUES (?1, ?2, ?3, ?4, 1)
         ON CONFLICT(article_id) DO UPDATE SET
           scroll_position=excluded.scroll_position,
           progress_percent=excluded.progress_percent,
           last_read_at=excluded.last_read_at,
           read_count=read_count + 0",
        params![article_id, scroll_position, progress_percent, now],
    )
    .map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub fn get_reading_progress(
    article_id: String,
    state: State<'_, DbState>,
) -> Result<Option<ReadingProgress>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let row = conn
        .query_row(
            "SELECT article_id, scroll_position, progress_percent, last_read_at, read_count
             FROM reading_progress WHERE article_id = ?1",
            params![article_id],
            |row| {
                Ok(ReadingProgress {
                    article_id: row.get(0)?,
                    scroll_position: row.get(1)?,
                    progress_percent: row.get(2)?,
                    last_read_at: row.get(3)?,
                    read_count: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(map_err)?;
    Ok(row)
}

#[tauri::command]
pub fn toggle_favorite(article_id: String, state: State<'_, DbState>) -> Result<bool, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let exists: Option<i64> = conn
        .query_row(
            "SELECT 1 FROM favorites WHERE article_id = ?1",
            params![article_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(map_err)?;
    if exists.is_some() {
        conn.execute("DELETE FROM favorites WHERE article_id = ?1", params![article_id])
            .map_err(map_err)?;
        Ok(false)
    } else {
        conn.execute(
            "INSERT INTO favorites (article_id, created_at) VALUES (?1, ?2)",
            params![article_id, now_ts()],
        )
        .map_err(map_err)?;
        Ok(true)
    }
}

#[tauri::command]
pub fn get_settings(state: State<'_, DbState>) -> Result<ReaderSettings, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let v: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'reader'",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(map_err)?;
    match v {
        Some(s) => Ok(serde_json::from_str(&s).unwrap_or_default()),
        None => Ok(ReaderSettings::default()),
    }
}

#[tauri::command]
pub fn update_settings(
    settings: ReaderSettings,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let json = serde_json::to_string(&settings).map_err(map_err)?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES ('reader', ?1)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![json],
    )
    .map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub fn set_kv(key: String, value: String, state: State<'_, DbState>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![key, value],
    )
    .map_err(map_err)?;
    Ok(())
}

#[tauri::command]
pub fn get_kv(key: String, state: State<'_, DbState>) -> Result<Option<String>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let v: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        )
        .optional()
        .map_err(map_err)?;
    Ok(v)
}

#[tauri::command]
pub fn set_always_on_top(enabled: bool, window: tauri::Window) -> Result<(), String> {
    window.set_always_on_top(enabled).map_err(map_err)
}

/// Classify selected text into "word" (single token) or "phrase" (multi-token).
fn classify_kind(text: &str) -> &'static str {
    let trimmed = text.trim();
    // Count whitespace-separated tokens. Treat hyphenated/apostrophe forms as one word.
    let token_count = trimmed.split_whitespace().count();
    if token_count <= 1 { "word" } else { "phrase" }
}

#[tauri::command]
pub fn add_note(
    article_id: Option<String>,
    text: String,
    context: Option<String>,
    state: State<'_, DbState>,
) -> Result<Note, String> {
    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        return Err("empty selection".into());
    }
    let kind = classify_kind(&trimmed).to_string();
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = now_ts();
    conn.execute(
        "INSERT INTO notes (article_id, kind, text, context, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![article_id, kind, trimmed, context, now],
    )
    .map_err(map_err)?;
    let id = conn.last_insert_rowid();

    // Resolve article title for display, if any.
    let article_title: Option<String> = if let Some(ref aid) = article_id {
        conn.query_row(
            "SELECT title FROM articles WHERE id = ?1",
            params![aid],
            |row| row.get(0),
      )
        .optional()
        .map_err(map_err)?
    } else {
        None
    };

    Ok(Note {
        id,
        article_id,
        kind,
        text: trimmed,
        context,
        created_at: now,
        article_title,
    })
}

#[tauri::command]
pub fn list_notes(
    kind: Option<String>,
    keyword: Option<String>,
    state: State<'_, DbState>,
) -> Result<Vec<Note>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut sql = String::from(
        "SELECT n.id, n.article_id, n.kind, n.text, n.context, n.created_at, a.title
         FROM notes n
         LEFT JOIN articles a ON a.id = n.article_id
         WHERE 1=1",
    );
    let mut args: Vec<String> = Vec::new();
    if let Some(k) = kind.filter(|s| !s.is_empty()) {
        sql.push_str(" AND n.kind = ?");
        args.push(k);
    }
    if let Some(kw) = keyword.filter(|s| !s.trim().is_empty()) {
        sql.push_str(" AND (n.text LIKE ? OR n.context LIKE ?)");
        let pat = format!("%{}%", kw);
        args.push(pat.clone());
        args.push(pat);
    }
    sql.push_str(" ORDER BY n.created_at DESC");

    let mut stmt = conn.prepare(&sql).map_err(map_err)?;
    let params_dyn: Vec<&dyn rusqlite::ToSql> =
        args.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
    let rows = stmt
        .query_map(params_dyn.as_slice(), |row| {
            Ok(Note {
                id: row.get(0)?,
                article_id: row.get(1).ok(),
                kind: row.get(2)?,
                text: row.get(3)?,
                context: row.get(4).ok(),
                created_at: row.get(5)?,
                article_title: row.get(6).ok(),
            })
        })
        .map_err(map_err)?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(map_err)?);
    }
    Ok(out)
}

#[tauri::command]
pub fn delete_note(id: i64, state: State<'_, DbState>) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])
        .map_err(map_err)?;
    Ok(())
}