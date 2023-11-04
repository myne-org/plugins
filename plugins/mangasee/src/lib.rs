use anyhow::Context;
use extism_pdk::*;
use interfaces::{metadata, Metadata, SearchResult, Series, SeriesStatus};
use scraper::{Html, Selector};

const BASE_URL: &'static str = "https://mangasee123.com";

#[plugin_fn]
pub fn metadata(_: ()) -> FnResult<Metadata> {
    Ok(metadata!(BASE_URL))
}

// #[plugin_fn]
pub fn search(_query: &str) -> FnResult<Vec<SearchResult>> {
    Ok(vec![])
}

pub fn get_series(id: String) -> FnResult<Series> {
    let page = reqwest::blocking::get(format!("{BASE_URL}/manga/{id}"))?.text()?;
    // some list item tags are incorrectly closed with </i> instead of </li>,
    // so we manually replace them here
    let page = page.replace("</i>", "</li>");
    let doc = Html::parse_document(&page);
    let title = doc
        .select(&Selector::parse("h1").unwrap())
        .next()
        .map(|e| e.text())
        .context("Should have h1 here")?
        .next()
        .context("Should have h1 here")?
        .trim()
        .to_string();

    let values = doc
        .select(&Selector::parse("li.list-group-item > a").unwrap())
        .map(|v| v.attr("href").unwrap().strip_prefix("/search/?"))
        .filter_map(|f| f.map(|t| t.split('=').collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let mut authors: Vec<String> = Vec::new();
    let mut genres: Vec<String> = Vec::new();
    let mut status = "";
    for value in values {
        match value[0] {
            "author" => authors.push(value[1].to_string()),
            "genre" => genres.push(value[1].to_string()),
            "status" => status = value[1],
            _ => {}
        }
    }
    let status = match status {
        "Ongoing" => SeriesStatus::Ongoing,
        "Completed" => SeriesStatus::Completed,
        _ => SeriesStatus::Cancelled,
    };
    let description = doc
        .select(&Selector::parse("li.list-group-item > div.top-5.Content").unwrap())
        .next()
        .context("should have gotten description")?
        .text()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let series = Series {
        extension_id: String::from(env!("CARGO_PKG_NAME")),
        id: id.clone(),
        title,
        authors,
        genres,
        description,
        synonyms: vec![],
        status,
        cover_url: format!("https://temp.compsci88.com/cover/${id}.jpg"),
    };
    Ok(series)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn search_test() {
    //     search("chainsaw").unwrap();
    // }

    #[test]
    fn get_series_test() {
        let one = get_series("Solo-Max-Level-Newbie".to_string()).unwrap();
        let two = get_series("Sensei-wa-Koi-o-Oshierarenai".to_string()).unwrap();
        println!("{one:#?} {two:#?}");
    }
}
