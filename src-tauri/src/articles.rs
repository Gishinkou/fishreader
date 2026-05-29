use crate::models::Article;
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Parse a markdown file with YAML-like front matter (simple parser, supports
/// scalar fields and list-of-strings tags using "- value" lines).
pub fn parse_markdown_file(path: &Path) -> Result<(Article, String)> {
    let raw = std::fs::read_to_string(path)?;
    let (fm, body) = split_front_matter(&raw);
    let mut id = String::new();
    let mut exam_type = String::from("TOEFL");
    let mut source: Option<String> = None;
    let mut title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string();
    let mut category: Option<String> = None;
    let mut tags: Vec<String> = Vec::new();
    let mut difficulty: Option<String> = None;

    let mut in_tags = false;
    for line in fm.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }
        if in_tags && line.trim_start().starts_with('-') {
            let v = line.trim_start().trim_start_matches('-').trim().to_string();
            if !v.is_empty() {
                tags.push(unquote(&v));
            }
            continue;
        }
        in_tags = false;
        if let Some(idx) = line.find(':') {
            let key = line[..idx].trim();
            let value = line[idx + 1..].trim();
            match key {
                "id" => id = unquote(value),
                "exam" | "examType" => exam_type = unquote(value),
                "source" => source = none_if_empty(unquote(value)),
                "title" => title = unquote(value),
                "category" => category = none_if_empty(unquote(value)),
                "difficulty" => difficulty = none_if_empty(unquote(value)),
                "tags" => {
                    if value.is_empty() {
                        in_tags = true;
                    } else if value.starts_with('[') && value.ends_with(']') {
                        let inner = &value[1..value.len() - 1];
                        for part in inner.split(',') {
                            let v = unquote(part.trim());
                            if !v.is_empty() {
                                tags.push(v);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if id.is_empty() {
        id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("article")
            .to_string();
    }

    let article = Article {
        id,
        exam_type,
        source,
        title,
        category,
        tags,
        difficulty,
        file_path: path.to_string_lossy().to_string(),
        is_favorite: false,
        progress_percent: 0.0,
        last_read_at: None,
    };
    Ok((article, body.to_string()))
}

fn split_front_matter(raw: &str) -> (&str, &str) {
    if let Some(stripped) = raw.strip_prefix("---\n") {
        if let Some(end) = stripped.find("\n---") {
            let fm = &stripped[..end];
            let rest_start = end + "\n---".len();
            let body = &stripped[rest_start..];
            let body = body.strip_prefix('\n').unwrap_or(body);
            return (fm, body);
        }
    }
    ("", raw)
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2)
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2)
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn none_if_empty(s: String) -> Option<String> {
    if s.is_empty() { None } else { Some(s) }
}

pub fn scan_articles_dir(dir: &PathBuf) -> Vec<(Article, String)> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            match parse_markdown_file(path) {
                Ok(v) => out.push(v),
                Err(e) => eprintln!("[articles] skip {:?}: {}", path, e),
            }
        }
    }
    out
}