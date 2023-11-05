use crate::{Chapter, Metadata, SearchResult, Series};
use extism_pdk::{FnResult, Json};

/// Wrapper type for plugin results
pub type PluginResult<T> = FnResult<Json<T>>;

/// A trait that all plugins should implement
///
/// Built in plugins will implement it themselves.
/// For wasm plugins used through extism, this might
/// need a wrapper from the host
pub trait PluginClient {
    /// Return plugin metadata
    fn metadata(&self) -> Metadata;

    /// Return search results based on `query`
    fn search(&self, query: String) -> PluginResult<Vec<SearchResult>>;

    /// Return series with `id`
    fn get_series(&self, id: String) -> PluginResult<Series>;

    /// Return chapter infos for series with `id`
    fn get_chapters(&self, id: String) -> PluginResult<Vec<Chapter>>;

    /// Return image urls for `chapter`
    fn get_images(&self, chapter: Chapter) -> PluginResult<Vec<String>>;
}
