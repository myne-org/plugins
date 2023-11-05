mod parse;

use interfaces::{metadata, PluginClient};
use reqwest::blocking::Client;

/// Plugin for Mangadex
#[derive(Debug, Clone)]
pub struct MangadexClient {
    pub(crate) client: Client,
}

impl MangadexClient {
    pub fn new() -> Self {
        MangadexClient {
            client: Client::new(),
        }
    }

    pub fn new_with_client(client: Client) -> Self {
        MangadexClient { client }
    }
}

// impl PluginClient for MangadexClient {
//     fn metadata(&self) -> interfaces::Metadata {
//         metadata!("https://mangadex.org")
//     }
//     fn get_series(&self, id: String) -> interfaces::PluginResult<interfaces::Series> {

//     }
// }
