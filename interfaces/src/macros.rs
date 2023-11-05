/// Generate a `Metadata` instance using values from
/// Cargo.toml
///
/// ```rs
/// let url = "https://some-site.com"
/// let is_deprecated = true;
/// let is_nsfw = true;
///
/// metadata!(url)
/// metadata!(url, is_deprecated)
/// metadata!(url, is_deprecated, is_nsfw)
/// ```
#[macro_export]
macro_rules! metadata {
    ($url:expr) => {
        $crate::metadata!($url, false, false)
    };
    ($url:expr, $deprecated:expr) => {
        $crate::metadata!($url, $deprecated, false)
    };
    ($url:expr, $deprecated:expr, $nsfw:expr) => {
        $crate::Metadata {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            description: String::from(env!("CARGO_PKG_DESCRIPTION")),
            url: String::from($url),
            deprecated: $deprecated,
            nsfw: $nsfw,
        }
    };
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn metadata_macro_test() {
        let generated = metadata!("https://mangadex.org", false, false);
        assert_eq!(
            generated,
            Metadata {
                name: String::from(env!("CARGO_PKG_NAME")),
                version: String::from(env!("CARGO_PKG_VERSION")),
                description: String::from(env!("CARGO_PKG_DESCRIPTION")),
                url: "https://mangadex.org".to_string(),
                nsfw: false,
                deprecated: false,
            }
        );
        println!("{:?}", generated);
    }
}
