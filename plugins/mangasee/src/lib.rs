use extism_pdk::*;
use headless::Browser;
use interfaces::{metadata, Metadata, SearchResult};
use scraper::{Html, Selector, ElementRef};
use url::Url;

const BASE_URL: &'static str = "https://mangasee123.com";

#[plugin_fn]
pub fn metadata(_: ()) -> FnResult<Metadata> {
    Ok(metadata!(BASE_URL))
}

// #[plugin_fn]
pub fn search(query: &str) -> FnResult<Json<Vec<SearchResult>>> {
    let browser = Browser::default()?;
    let page = browser.new_tab()?;
    page.navigate_to(
        Url::parse_with_params(format!("{BASE_URL}/search/").as_str(), [("name", query)])?.as_str(),
    )?
    .wait_until_navigated()?;

    let doc = Html::parse_document(&page.get_content()?);
    let res: Vec<_> = doc
        .select(&Selector::parse(".top-15.ng-scope").unwrap())
        .map(|s| {
            let first = s
                .select(&Selector::parse(r#".SeriesName[ng-bind-html="Series.s"]"#).unwrap())
                .next()
                .unwrap();
            // (first.text(), first.attr("href"))
            first.html()
        })
        .collect();
    println!("{:?}", res);
    Ok(Json(vec![]))
}

pub fn get_series(id: String) -> FnResult<Json<()>> {
    let page = reqwest::blocking::get(format!("{BASE_URL}/manga/{id}"))?.text()?;
    // some list item tags are incorrectly closed with </i> instead of </li>,
    // so we manually replace them here
    let page = page.replace("</i>", "</li>");
    let doc = Html::parse_document(&page);
    let _title = doc
        .select(&Selector::parse("h1").unwrap())
        .next()
        .map(|e| e.text())
        .unwrap()
        .next()
        .unwrap()
        .trim()
        .to_string();
    let mlabel = Selector::parse(".mlabel").unwrap();
    let labels: Vec<ElementRef> = doc.select(&mlabel).collect();
    let mut authors: Vec<String> = vec![];
    let mut genres: Vec<String> = vec![];
    labels
    // for element in detail_labels {

    // }
    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn search_test() {
    //     search("chainsaw").unwrap();
    // }
}
