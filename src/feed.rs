use crate::{
    config::{Article, Config, Rss},
    error::Error,
};
use glob::glob;
use jiff::{Timestamp, civil::Date, fmt::rfc2822};
use rss::{Channel, Enclosure, Guid, Image, Item};
use scraper::{Html, Selector};
use std::{
    cmp,
    ffi::OsStr,
    io::ErrorKind,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{fs, task::JoinSet};
use tracing::{Instrument, info, info_span, warn};
use url::Url;

/// Compiled CSS selectors used to scrape metadata from article HTML documents.
struct Selectors {
    title: Selector,
    description: Selector,
    content: Selector,
    date: Selector,
}

impl Selectors {
    fn parse(article: &Article) -> Result<Self, Error> {
        Ok(Self {
            title: parse_selector(&article.title)?,
            description: parse_selector(&article.description)?,
            content: parse_selector(&article.content)?,
            date: parse_selector(&article.date)?,
        })
    }
}

/// Shared read-only context across concurrent article-building tasks.
struct ArticleContext {
    site_url: Url,
    article: Article,
    selectors: Selectors,
}

fn parse_selector(raw: &str) -> Result<Selector, Error> {
    Selector::parse(raw).map_err(|_| Error::Selector(raw.to_owned()))
}

/// Build the RSS feed based on the properties in `config_path`, write it to disk,
/// and return the generated [`Channel`].
///
/// # Examples
///
/// ```no_run
/// use anyhow::Result;
/// use std::path::Path;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     katharsis::build(Path::new("katharsis.toml")).await?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// - When the configuration file cannot be loaded or parsed.
/// - When any CSS selector in the configuration is invalid.
/// - When the glob pattern is invalid or a matched path cannot be read.
/// - When a concurrent article-processing task fails to complete.
/// - When article files or image metadata cannot be read.
/// - When an article's publication date cannot be parsed.
/// - When the output RSS file cannot be written to disk.
pub async fn build(config_path: &Path) -> Result<Channel, Error> {
    let Config { mut rss, article } = Config::load(config_path).await?;
    let selectors = Selectors::parse(&article)?;
    let sort = article.sort;
    let ctx = Arc::new(ArticleContext {
        site_url: rss.site_url.clone(),
        article,
        selectors,
    });

    let mut set = JoinSet::new();
    for (idx, entry) in glob(&ctx.article.input)?.enumerate() {
        let path = entry?;
        let ctx = Arc::clone(&ctx);
        let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        let span = info_span!("article", file = %file_name);

        set.spawn(
            async move {
                let item = build_item(&path, &ctx.site_url, &ctx.article, &ctx.selectors).await?;
                Ok::<(usize, Item), Error>((idx, item))
            }
            .instrument(span),
        );
    }

    if set.is_empty() {
        warn!(input = %ctx.article.input, "no articles matched the input pattern");
    }

    let mut indexed_items = Vec::with_capacity(set.len());
    while let Some(res) = set.join_next().await {
        indexed_items.push(res??);
    }
    indexed_items.sort_unstable_by_key(|(idx, _)| *idx);

    let mut items: Vec<Item> = indexed_items.into_iter().map(|(_, item)| item).collect();
    if sort {
        sort_items(&mut items);
    }

    info!(output = %rss.output.display(), "creating RSS feed");

    let output_path = mem::take(&mut rss.output);
    let channel = build_channel(rss, items);
    fs::write(&output_path, channel.to_string()).await?;

    Ok(channel)
}

async fn build_item(
    path: &Path,
    site_url: &Url,
    article: &Article,
    selectors: &Selectors,
) -> Result<Item, Error> {
    let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap_or_default();

    info!("parsing article");

    let article_url = format!("{site_url}{}/{file_stem}", article.prefix);
    let image_path = PathBuf::from(article.image.replace("**", file_stem));
    let enclosure = build_enclosure(&article_url, &image_path).await?;

    let mut item = Item {
        author: Some(format!(
            "{} ({})",
            article.author.email, article.author.name
        )),
        link: Some(article_url.clone()),
        guid: Some(Guid {
            value: article_url,
            permalink: true,
        }),
        enclosure,
        ..Default::default()
    };

    let html = fs::read_to_string(path).await?;
    populate_html_metadata(&html, selectors, &mut item)?;

    Ok(item)
}

async fn build_enclosure(article_url: &str, image_path: &Path) -> Result<Option<Enclosure>, Error> {
    let metadata = match fs::metadata(image_path).await {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            warn!(path = %image_path.display(), "image not found");
            return Ok(None);
        }
        Err(err) => return Err(Error::Io(err)),
    };

    let file_name = image_path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or_default();
    let extension = image_path
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default();

    Ok(Some(Enclosure {
        url: format!("{article_url}/{file_name}"),
        length: metadata.len().to_string(),
        mime_type: format!("image/{extension}"),
    }))
}

fn populate_html_metadata(html: &str, selectors: &Selectors, item: &mut Item) -> Result<(), Error> {
    let document = Html::parse_document(html);

    if let Some(title) = extract_text(&document, &selectors.title) {
        item.set_title(title);
    } else {
        warn!("title element not found");
    }

    if let Some(description) = extract_text(&document, &selectors.description) {
        item.set_description(description);
    } else {
        warn!("description element not found");
    }

    if let Some(content) = extract_html(&document, &selectors.content) {
        item.set_content(content);
    } else {
        warn!("content element not found");
    }

    if let Some(pub_date) = extract_pub_date(&document, &selectors.date)? {
        item.set_pub_date(pub_date);
    } else {
        warn!("date element not found");
    }

    Ok(())
}

fn extract_text(document: &Html, selector: &Selector) -> Option<String> {
    document.select(selector).next().map(|element| {
        element
            .text()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn extract_html(document: &Html, selector: &Selector) -> Option<String> {
    document
        .select(selector)
        .next()
        .map(|element| element.inner_html())
}

fn extract_pub_date(document: &Html, selector: &Selector) -> Result<Option<String>, Error> {
    let Some(element) = document.select(selector).next() else {
        return Ok(None);
    };
    let Some(datetime) = element.value().attr("datetime") else {
        return Ok(None);
    };

    format_pub_date(datetime).map(Some)
}

fn format_pub_date(datetime: &str) -> Result<String, Error> {
    let date = Date::strptime("%Y-%m-%d", datetime)?;
    Ok(date.strftime("%a, %d %b %Y 00:00:00 GMT").to_string())
}

fn sort_items(items: &mut [Item]) {
    items.sort_by_cached_key(|item| {
        cmp::Reverse(
            item.pub_date
                .as_deref()
                .and_then(|d| rfc2822::parse(d).ok()),
        )
    });
}

fn build_channel(rss: Rss, items: Vec<Item>) -> Channel {
    let link = String::from(rss.site_url);
    let image = Image {
        url: format!("{link}{}", rss.image),
        title: rss.title.clone(),
        link: link.clone(),
        ..Default::default()
    };

    Channel {
        title: rss.title,
        description: rss.description,
        link,
        image: Some(image),
        copyright: Some(rss.copyright),
        language: Some(rss.language),
        items,
        docs: Some(String::from("https://validator.w3.org/feed/docs/rss2.html")),
        generator: Some(String::from("https://github.com/arghena/katharsis")),
        last_build_date: Some(
            Timestamp::now()
                .strftime("%a, %d %b %Y %H:%M:%S GMT")
                .to_string(),
        ),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Author;
    use tempfile::tempdir;
    use tokio_test::{assert_err, assert_ok};

    fn sample_article_config() -> Article {
        Article {
            title: String::from("h1"),
            description: String::from("[data-mdx-description]"),
            input: String::from("*.html"),
            author: Author {
                name: String::from("Vercel Inc"),
                email: String::from("nextjs@vercel.com"),
            },
            prefix: String::from("articles"),
            content: String::from("[data-mdx-content]"),
            date: String::from("time"),
            image: String::from("**/opengraph-image.png"),
            sort: true,
        }
    }

    #[test]
    fn parse_selector_valid_and_invalid() {
        assert_ok!(parse_selector("h1"));
        assert_ok!(parse_selector("[data-description]"));

        let err = assert_err!(parse_selector("::invalid-pseudo"));
        assert!(matches!(err, Error::Selector(s) if s == "::invalid-pseudo"));
    }

    #[test]
    fn selectors_parse_from_article() {
        let article = sample_article_config();
        assert_ok!(Selectors::parse(&article));
    }

    #[test]
    fn extract_text_handles_present_and_missing_elements() {
        let html = Html::parse_document("<h1>Hello <span>Rust</span></h1>");
        let h1 = assert_ok!(parse_selector("h1"));
        let h2 = assert_ok!(parse_selector("h2"));

        assert_eq!(extract_text(&html, &h1), Some(String::from("Hello Rust")));
        assert_eq!(extract_text(&html, &h2), None);
    }

    #[test]
    fn extract_html_returns_inner_markup() {
        let html = Html::parse_document("<article data-mdx-content><p>Body</p></article>");
        let selector = assert_ok!(parse_selector("[data-mdx-content]"));
        let missing = assert_ok!(parse_selector("section"));

        assert_eq!(
            extract_html(&html, &selector),
            Some(String::from("<p>Body</p>"))
        );
        assert_eq!(extract_html(&html, &missing), None);
    }

    #[test]
    fn format_pub_date_valid_and_invalid() {
        let formatted = assert_ok!(format_pub_date("2024-10-24"));
        assert_eq!(formatted, "Thu, 24 Oct 2024 00:00:00 GMT");

        let err = assert_err!(format_pub_date("24-10-2024"));
        assert!(matches!(err, Error::Date(_)));
    }

    #[test]
    fn extract_pub_date_handles_all_cases() {
        let selector = assert_ok!(parse_selector("time"));

        let with_attr = Html::parse_document(r#"<time datetime="2024-10-24">Oct 24</time>"#);
        assert_eq!(
            assert_ok!(extract_pub_date(&with_attr, &selector)),
            Some(String::from("Thu, 24 Oct 2024 00:00:00 GMT"))
        );

        let without_attr = Html::parse_document("<time>Oct 24</time>");
        assert_eq!(assert_ok!(extract_pub_date(&without_attr, &selector)), None);

        let without_element = Html::parse_document("<p>No time</p>");
        assert_eq!(
            assert_ok!(extract_pub_date(&without_element, &selector)),
            None
        );
    }

    #[test]
    fn populate_html_metadata_handles_full_and_empty_documents() {
        let selectors = assert_ok!(Selectors::parse(&sample_article_config()));

        let full_html = r#"
            <h1>Beta Article Title</h1>
            <p data-mdx-description="true">Beta article description.</p>
            <div data-mdx-content="true"><p>Content</p></div>
            <time datetime="2024-10-24">October 24th, 2024</time>
        "#;
        let mut item = Item::default();
        assert_ok!(populate_html_metadata(full_html, &selectors, &mut item));

        assert_eq!(item.title(), Some("Beta Article Title"));
        assert_eq!(item.description(), Some("Beta article description."));
        assert_eq!(item.content(), Some("<p>Content</p>"));
        assert_eq!(item.pub_date(), Some("Thu, 24 Oct 2024 00:00:00 GMT"));

        let mut empty_item = Item::default();
        assert_ok!(populate_html_metadata(
            "<div></div>",
            &selectors,
            &mut empty_item
        ));
        assert!(empty_item.title().is_none());
        assert!(empty_item.description().is_none());
        assert!(empty_item.content().is_none());
        assert!(empty_item.pub_date().is_none());
    }

    #[tokio::test]
    async fn build_enclosure_when_image_exists_and_missing() {
        let dir = assert_ok!(tempdir());
        let img_path = dir.path().join("opengraph-image.png");
        let dummy_bytes = b"fake-png-content";
        assert_ok!(fs::write(&img_path, dummy_bytes).await);

        let enclosure =
            assert_ok!(build_enclosure("https://nextjs.org/articles/beta", &img_path).await);
        assert_eq!(
            enclosure,
            Some(Enclosure {
                url: String::from("https://nextjs.org/articles/beta/opengraph-image.png"),
                length: dummy_bytes.len().to_string(),
                mime_type: String::from("image/png"),
            })
        );

        let missing_path = dir.path().join("missing.png");
        let none_enclosure =
            assert_ok!(build_enclosure("https://nextjs.org/articles/beta", &missing_path).await);
        assert!(none_enclosure.is_none());
    }

    #[test]
    fn sort_items_orders_newest_first() {
        let mut items = vec![
            Item {
                title: Some(String::from("Older")),
                pub_date: Some(String::from("Mon, 21 Oct 2024 00:00:00 GMT")),
                ..Default::default()
            },
            Item {
                title: Some(String::from("Newer")),
                pub_date: Some(String::from("Thu, 24 Oct 2024 00:00:00 GMT")),
                ..Default::default()
            },
        ];

        sort_items(&mut items);

        assert_eq!(items[0].title(), Some("Newer"));
        assert_eq!(items[1].title(), Some("Older"));
    }

    #[test]
    fn build_channel_constructs_valid_rss_channel() {
        let rss = Rss {
            title: String::from("The React Framework for the Web"),
            description: String::from("Next.js description"),
            site_url: assert_ok!(Url::parse("https://nextjs.org")),
            image: String::from("favicon.png"),
            copyright: String::from("© 2024 Vercel Inc"),
            language: String::from("en"),
            output: PathBuf::from("rss.xml"),
        };

        let channel = build_channel(rss, Vec::new());

        assert_eq!(channel.title(), "The React Framework for the Web");
        assert_eq!(channel.description(), "Next.js description");
        assert_eq!(channel.link(), "https://nextjs.org/");
        assert_eq!(
            channel.image(),
            Some(&Image {
                url: String::from("https://nextjs.org/favicon.png"),
                title: String::from("The React Framework for the Web"),
                link: String::from("https://nextjs.org/"),
                ..Default::default()
            })
        );
        assert_eq!(channel.copyright(), Some("© 2024 Vercel Inc"));
        assert_eq!(channel.language(), Some("en"));
        assert_eq!(
            channel.docs(),
            Some("https://validator.w3.org/feed/docs/rss2.html")
        );
        assert_eq!(
            channel.generator(),
            Some("https://github.com/arghena/katharsis")
        );
        let last_build_date = channel
            .last_build_date()
            .expect("lastBuildDate should be set");
        assert_ok!(rfc2822::parse(last_build_date));
    }

    #[tokio::test]
    async fn build_end_to_end_with_tempdir() {
        let dir = assert_ok!(tempdir());
        let articles_dir = dir.path().join("articles");
        let beta_img_dir = dir.path().join("images/beta");
        assert_ok!(fs::create_dir_all(&articles_dir).await);
        assert_ok!(fs::create_dir_all(&beta_img_dir).await);

        let alpha_html = r#"
            <h1>Alpha Article Title</h1>
            <p data-mdx-description="true">Alpha article description.</p>
            <div data-mdx-content="true"><p>Alpha content.</p></div>
            <time datetime="2024-10-21">October 21, 2024</time>
        "#;
        let beta_html = r#"
            <h1>Beta Article Title</h1>
            <p data-mdx-description="true">Beta article description.</p>
            <div data-mdx-content="true"><p>Beta content.</p></div>
            <time datetime="2024-10-24">October 24, 2024</time>
        "#;
        assert_ok!(fs::write(articles_dir.join("alpha.html"), alpha_html).await);
        assert_ok!(fs::write(articles_dir.join("beta.html"), beta_html).await);
        assert_ok!(fs::write(beta_img_dir.join("opengraph-image.png"), b"image_bytes").await);

        let output_path = dir.path().join("rss.xml");
        let dir_str = dir.path().to_string_lossy().replace('\\', "/");
        let output_str = output_path.to_string_lossy().replace('\\', "/");

        let config_toml = format!(
            r#"
            [rss]
            title = "The React Framework for the Web"
            description = "Next.js enables you to create high-quality web applications."
            site_url = "https://nextjs.org"
            image = "favicon.png"
            copyright = "© 2024 Vercel Inc"
            language = "en"
            output = "{output_str}"

            [article]
            title = "h1"
            description = "[data-mdx-description]"
            input = "{dir_str}/articles/*.html"
            author = {{ name = "Vercel Inc", email = "nextjs@vercel.com" }}
            prefix = "articles"
            content = "[data-mdx-content]"
            date = "time"
            image = "{dir_str}/images/**/opengraph-image.png"
            sort = true
            "#
        );

        let config_path = dir.path().join("katharsis.toml");
        assert_ok!(fs::write(&config_path, config_toml).await);

        let channel = assert_ok!(build(&config_path).await);

        assert_eq!(channel.items().len(), 2);
        assert_eq!(channel.items()[0].title(), Some("Beta Article Title"));
        assert_eq!(channel.items()[1].title(), Some("Alpha Article Title"));
        assert!(channel.items()[0].enclosure().is_some());
        assert!(channel.items()[1].enclosure().is_none());
        assert!(assert_ok!(output_path.try_exists()));
    }

    #[tokio::test]
    async fn build_warns_when_no_articles_match() {
        let dir = assert_ok!(tempdir());
        let output_path = dir.path().join("rss.xml");
        let dir_str = dir.path().to_string_lossy().replace('\\', "/");
        let output_str = output_path.to_string_lossy().replace('\\', "/");

        let config_toml = format!(
            r#"
            [rss]
            title = "Empty Feed"
            description = "No articles."
            site_url = "https://example.com"
            image = "favicon.png"
            copyright = "© 2024"
            language = "en"
            output = "{output_str}"

            [article]
            title = "h1"
            description = "p"
            input = "{dir_str}/non_existent/*.html"
            author = {{ name = "Author", email = "author@example.com" }}
            prefix = "articles"
            content = "div"
            date = "time"
            image = "{dir_str}/images/**/image.png"
            sort = false
            "#
        );

        let config_path = dir.path().join("katharsis.toml");
        assert_ok!(fs::write(&config_path, config_toml).await);

        let channel = assert_ok!(build(&config_path).await);
        assert_eq!(channel.items(), []);
    }
}
