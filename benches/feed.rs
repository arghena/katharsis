//! Benchmarks for Katharsis configuration parsing, initialization, and RSS feed generation.

use divan::{AllocProfiler, Bencher, black_box};
use katharsis::{Config, config::DEFAULT_TEMPLATE};
use std::{fs, path::PathBuf, str::FromStr};
use tempfile::{TempDir, tempdir};
use tokio::runtime::Runtime;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

/// Benchmark in-memory TOML configuration deserialization and URL validation.
#[divan::bench]
fn parse_config() -> Config {
    Config::from_str(black_box(DEFAULT_TEMPLATE)).expect("failed to parse default template")
}

/// Benchmark asynchronous configuration file initialization (`katharsis init -y`).
#[divan::bench]
fn init_config(bencher: Bencher) {
    let rt = Runtime::new().expect("failed to build Tokio runtime");
    let dir = tempdir().expect("failed to create temp dir");
    let config_path = dir.path().join("katharsis.toml");

    bencher.bench_local(|| {
        rt.block_on(katharsis::init(black_box(&config_path), true))
            .expect("failed to initialize config");
    });
}

/// Benchmark end-to-end concurrent RSS feed generation across different article counts.
#[divan::bench(args = [1, 10, 50])]
fn build_feed(bencher: Bencher, article_count: usize) {
    let rt = Runtime::new().expect("failed to build Tokio runtime");
    let (_dir, config_path) = setup_workspace(article_count);

    bencher.bench_local(|| {
        rt.block_on(katharsis::build(black_box(&config_path)))
            .expect("failed to build RSS feed")
    });
}

fn setup_workspace(article_count: usize) -> (TempDir, PathBuf) {
    let dir = tempdir().expect("failed to create temp dir");
    let articles_dir = dir.path().join("articles");
    fs::create_dir_all(&articles_dir).expect("failed to create articles dir");

    for i in 0..article_count {
        let slug = format!("article-{i:03}");
        let day = (i % 28) + 1;
        let html = format!(
            r#"
            <h1>Article Title {i}</h1>
            <p data-mdx-description="true">Description for article {i}.</p>
            <div data-mdx-content="true"><p>Content body for article {i}.</p></div>
            <time datetime="2024-10-{day:02}">October {day}, 2024</time>
            "#
        );
        fs::write(articles_dir.join(format!("{slug}.html")), html)
            .expect("failed to write article html");

        let img_dir = dir.path().join("images").join(&slug);
        fs::create_dir_all(&img_dir).expect("failed to create image dir");
        fs::write(img_dir.join("opengraph-image.png"), b"fake_png_bytes")
            .expect("failed to write image");
    }

    let output_path = dir.path().join("rss.xml");
    let dir_str = dir.path().to_string_lossy().replace('\\', "/");
    let output_str = output_path.to_string_lossy().replace('\\', "/");

    let config_toml = format!(
        r#"
        [rss]
        title = "Benchmark Feed"
        description = "Benchmarking concurrent RSS generation."
        site_url = "https://example.com"
        image = "favicon.png"
        copyright = "© 2024 Benchmark"
        language = "en"
        output = "{output_str}"

        [article]
        title = "h1"
        description = "[data-mdx-description]"
        input = "{dir_str}/articles/*.html"
        author = {{ name = "Bench Author", email = "bench@example.com" }}
        prefix = "articles"
        content = "[data-mdx-content]"
        date = "time"
        image = "{dir_str}/images/**/opengraph-image.png"
        sort = true
        "#
    );

    let config_path = dir.path().join("katharsis.toml");
    fs::write(&config_path, config_toml).expect("failed to write config");

    (dir, config_path)
}
