use crate::{structs::SearchResult, Chapter};
use extism_pdk::FnResult;

/// Search function signature
/// Function name: `search`
///
/// Return a vec of series that match the search term
/// The search term should be provided as argument
pub type SearchFn = dyn Fn(String) -> FnResult<Vec<SearchResult>>;

/// Get chapter function signature
/// Function name: `get_chapters`
///
/// Return all chapters of a series
pub type GetChaptersFn = dyn Fn(String) -> FnResult<Vec<Chapter>>;

pub type GetInfoFn = dyn Fn(String) -> FnResult<()>;
