use extism_pdk::*;
use headless::Browser;
use interfaces::{metadata, Metadata, SearchResult};

const BASE_URL: &'static str = "https://mangasee123.com";

#[plugin_fn]
pub fn metadata(_: ()) -> FnResult<Metadata> {
    Ok(metadata!(BASE_URL))
}

#[plugin_fn]
pub fn search(query: String) -> FnResult<Json<Vec<SearchResult>>> {
    let browser = Browser::default()?;
    let page = browser.new_tab()?;
    page.navigate_to(format!("{BASE_URL}/search/?name={query}").as_str())?.wait_until_navigated()?;
    Ok(Json(vec![]))
}

#[cfg(test)]
mod tests {}
