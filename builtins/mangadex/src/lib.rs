use reqwest::blocking::Client;

/// Plugin for Mangadex
#[derive(Debug, Clone)]
pub struct MangadexClient {
    _client: Client,
}
