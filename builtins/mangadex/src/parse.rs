use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

/// parse a json response into a manga
pub(crate) fn parse_manga(json: Value) -> anyhow::Result<Manga> {
    if let Some(result) = json.get("result") {
        if !result.as_str().is_some_and(|f| f == "ok") || json.get("data").is_none() {
            anyhow::bail!("Response failed. Got {json}")
        }
    } else {
        anyhow::bail!("Response failed. Did not receive ok. Got {json}")
    }
    let data = json.get("data").unwrap();
    Ok(serde_json::from_value(data.to_owned())?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manga {
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attributes {
    pub title: HashMap<String, String>,
    #[serde(rename = "altTitles")]
    pub alt_titles: Vec<HashMap<String, String>>,
    pub description: HashMap<String, String>,
    pub status: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    pub id: String,
    pub attributes: TagAttribute,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagAttribute {
    pub name: HashMap<String, String>,
}
