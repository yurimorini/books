use super::Isbn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Volume {
    pub isbn: Isbn,
    pub title: String,
    pub description: String,
    pub publisher: String,
    pub published_date: String,
    pub image: String,
    pub language: String,
    pub authors: Vec<String>,
    pub pages: i64,
}
