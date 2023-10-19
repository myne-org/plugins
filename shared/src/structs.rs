/// Results returned from the `search` function
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