//! CLI tool and library for generating RSS 2.0 feeds from static HTML files.
//!
//! # Examples
//!
//! ```no_run
//! use anyhow::Result;
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config_path = Path::new("katharsis.toml");
//!
//!     katharsis::init(config_path, false).await?;
//!     katharsis::build(config_path).await?;
//!
//!     Ok(())
//! }
//! ```

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

/// Configuration models and TOML loader.
pub mod config;
/// Error types returned by Katharsis operations.
pub mod error;
/// RSS feed scraping and generation logic.
pub mod feed;
/// Configuration file initialization logic.
pub mod init;

pub use config::Config;
pub use error::Error;
pub use feed::build;
pub use init::init;
