use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    #[serde(rename = "examType")]
    pub exam_type: String,
    pub source: Option<String>,
    pub title: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub difficulty: Option<String>,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "isFavorite")]
    pub is_favorite: bool,
    #[serde(rename = "progressPercent")]
    pub progress_percent: f64,
    #[serde(rename = "lastReadAt")]
    pub last_read_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleContent {
    pub article: Article,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderSettings {
    pub theme: String,
    #[serde(rename = "fontSize")]
    pub font_size: f64,
    #[serde(rename = "lineHeight")]
    pub line_height: f64,
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "alwaysOnTop")]
    pub always_on_top: bool,
}

impl Default for ReaderSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            font_size: 17.0,
            line_height: 1.75,
            font_family: "system".to_string(),
            always_on_top: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingProgress {
    #[serde(rename = "articleId")]
    pub article_id: String,
    #[serde(rename = "scrollPosition")]
    pub scroll_position: f64,
    #[serde(rename = "progressPercent")]
    pub progress_percent: f64,
    #[serde(rename = "lastReadAt")]
    pub last_read_at: i64,
    #[serde(rename = "readCount")]
    pub read_count: i64,
}