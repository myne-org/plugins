use serde::{Deserialize, Serialize};

/// Results returned from the `search` function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Title of the series
    pub title: String,
    /// Url to the series
    pub url: String,
    /// Url for the thumbnail image
    pub thumbnail: String,
    /// Series locale
    pub locale: String,
}

/// A chapter entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Chapter id
    pub id: String,
    /// The id of the series it belongs to
    pub series_id: String,
    /// Chapter name
    pub title: String,
    /// Chapter number
    pub chapter_number: usize,
    /// Volume number, should be set to 0 if not applicable
    pub volume_number: usize,
    /// Url to the chapter
    pub url: String,
}

/// Represents info about a series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: String,
    pub extension_id: String,
    pub source_id: String,
    pub title: String,
    pub synonyms: Vec<String>,
}
