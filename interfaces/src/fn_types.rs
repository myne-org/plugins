use crate::{Chapter, SearchResult, Series, Metadata};
use extism_pdk::FnResult;

/// Get metadata function signature
/// Function name: `metadata`
pub type GetMetadataFn = dyn Fn() -> FnResult<Metadata>;

/// Search function signature
/// Function name: `search`
///
/// Return a vec of series that match the search term
/// The search term should be provided as argument
pub type SearchFn = dyn Fn(String) -> FnResult<Vec<SearchResult>>;

/// Get Series function signature
/// Function name: `get_series`
///
/// Returns info related to a series. The argument passed is the series ID
pub type GetSeriesFn = dyn Fn(String) -> FnResult<Series>;

/// Get chapter function signature
/// Function name: `get_chapters`
///
/// Return all chapters of a series. The argument passed is the series ID
pub type GetChaptersFn = dyn Fn(String) -> FnResult<Vec<Chapter>>;

/// Get images function signature
/// Function name: `get_images`
///
/// Returns a vec of urls of all the images for a chapter. The argument passed is the chapter entry
pub type GetImagesFn = dyn Fn(Chapter) -> FnResult<Vec<String>>;
