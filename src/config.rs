//! Configuration models and TOML loader.

use crate::error::Error;
use serde::Deserialize;
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::fs;
use tracing::error;
use url::Url;

/// Embedded default configuration template (`katharsis.toml`).
pub const DEFAULT_TEMPLATE: &str = include_str!("katharsis.toml");

/// Root configuration structure parsed from `katharsis.toml`.
///
/// # Examples
///
/// Parsing a [`Config`] from a TOML string in memory:
///
/// ```
/// use katharsis::Config;
/// use katharsis::config::DEFAULT_TEMPLATE;
/// use std::str::FromStr;
///
/// let config = Config::from_str(DEFAULT_TEMPLATE)?;
/// assert_eq!(config.rss.language, "en");
/// assert!(config.article.sort);
/// # Ok::<(), katharsis::Error>(())
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// RSS channel configuration properties.
    pub rss: Rss,
    /// Article scraping and discovery configuration properties.
    pub article: Article,
}

/// RSS channel configuration properties.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Rss {
    /// The title of the website.
    pub title: String,
    /// A description of the website.
    pub description: String,
    /// The main URL of the website.
    pub site_url: Url,
    /// Path to the channel's logo relative to `site_url`.
    pub image: String,
    /// Copyright information.
    pub copyright: String,
    /// Preferred language.
    pub language: String,
    /// Path to the local output RSS file.
    pub output: PathBuf,
}

/// Article scraping and discovery configuration properties.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Article {
    /// The tag containing the article's title.
    pub title: String,
    /// The tag or attribute containing the article's description.
    pub description: String,
    /// The HTML files that need to be parsed.
    pub input: String,
    /// Information about the article's author.
    pub author: Author,
    /// The relative prefix for the article's URL.
    pub prefix: String,
    /// The tag or attribute containing the article's content.
    pub content: String,
    /// The tag containing the article's publication date.
    pub date: String,
    /// The image files to be used as the article cover.
    pub image: String,
    /// Whether to sort articles by their publication date.
    pub sort: bool,
}

/// Article author metadata.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Author {
    /// The author's display name.
    pub name: String,
    /// The author's email address.
    pub email: String,
}

impl FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}

impl Config {
    /// Load and deserialize the configuration file from the given `path`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use katharsis::Config;
    /// use std::path::Path;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), katharsis::Error> {
    ///     let config = Config::load(Path::new("katharsis.toml")).await?;
    ///     println!("Title: {}", config.rss.title);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// - When the file does not exist ([`Error::FileNotFound`]).
    /// - When the file cannot be read.
    /// - When the TOML content cannot be deserialized.
    pub async fn load(path: &Path) -> Result<Self, Error> {
        let content = match fs::read_to_string(path).await {
            Ok(content) => content,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                error!(config = %path.display(), "configuration file does not exist");
                return Err(Error::FileNotFound);
            }
            Err(err) => return Err(Error::Io(err)),
        };

        Self::from_str(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio_test::{assert_err, assert_ok};

    #[test]
    fn parse_default_template() {
        let Config { rss, article } = assert_ok!(Config::from_str(DEFAULT_TEMPLATE));

        assert_eq!(rss.title, "Your Website Title");
        assert_eq!(rss.description, "Your Website Description");
        assert_eq!(rss.site_url.as_str(), "https://example.com/");
        assert_eq!(rss.image, "favicon.png");
        assert_eq!(rss.copyright, "© Year Your Name");
        assert_eq!(rss.language, "en");
        assert_eq!(rss.output, PathBuf::from("rss.xml"));

        assert_eq!(article.title, "h1");
        assert_eq!(article.description, "[data-description]");
        assert_eq!(article.input, "articles/*.html");
        assert_eq!(article.author.name, "Your Name");
        assert_eq!(article.author.email, "Your Email");
        assert_eq!(article.prefix, "articles");
        assert_eq!(article.content, "[data-content]");
        assert_eq!(article.date, "time");
        assert_eq!(article.image, "articles/**/image.png");
        assert!(article.sort);
    }

    #[test]
    fn parse_invalid_toml_returns_error() {
        let err = assert_err!(Config::from_str("invalid_toml = ["));
        assert!(matches!(err, Error::TomlDeserialize(_)));
    }

    #[tokio::test]
    async fn load_existing_config_file() {
        let dir = assert_ok!(tempdir());
        let path = dir.path().join("katharsis.toml");
        assert_ok!(fs::write(&path, DEFAULT_TEMPLATE).await);

        let config = assert_ok!(Config::load(&path).await);
        let expected = assert_ok!(Config::from_str(DEFAULT_TEMPLATE));
        assert_eq!(config, expected);
    }

    #[tokio::test]
    async fn load_non_existent_file_returns_error() {
        let dir = assert_ok!(tempdir());
        let path = dir.path().join("missing.toml");

        let err = assert_err!(Config::load(&path).await);
        assert!(matches!(err, Error::FileNotFound));

        let io_err = assert_err!(Config::load(Path::new("invalid\0path.toml")).await);
        assert!(matches!(io_err, Error::Io(_)));
    }
}
