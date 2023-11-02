use serde::{Deserialize, Serialize};

/// Metadata of a plugin
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    /// Extension name.
    /// This essentially serves as the extension's unique identifier
    pub name: String,
    /// Current version of the extension
    pub version: String,
    /// Description of the extension
    pub description: String,
    /// Url to the site this extension is for
    pub url: String,
    /// Mark as true if the extension is deprecated/unmaintained for some reason
    pub deprecated: bool,
    /// If the extension is for adult sites or sites that mostly
    /// provide nsfw series
    pub nsfw: bool,
}

/// Results returned from the `search` function
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    /// Series ID
    pub id: String,
    /// Title of the series
    pub title: String,
    /// Url to the series
    pub url: String,
    /// Url for the thumbnail image
    pub thumbnail: String,
    /// Series language
    pub language: String,
}

/// A chapter entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    /// Chapter id
    pub id: String,
    /// The id of the series it belongs to
    pub series_id: String,
    /// Chapter name
    ///
    /// If there no chapter name, use chapter number as name
    pub title: String,
    /// Chapter number
    pub chapter_number: usize,
    /// Volume number
    pub volume_number: Option<usize>,
    /// Url to the chapter
    pub url: String,
}

/// Represents info about a series
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Series {
    /// Series ID
    ///
    /// This should be a unique identifier which if provided multiple times to
    /// the plugin, will always return the same series
    pub id: String,
    /// The extension's id that is returning this.
    pub extension_id: String,
    /// Series title
    pub title: String,
    /// Other names of the series
    pub synonyms: Vec<String>,
    /// Genres of the series. Return empty vec if not applicable
    pub genres: Vec<String>,
}
