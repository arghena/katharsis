//! CLI tool and library for generating RSS 2.0 feeds from static HTML files.
//!
//! # Examples
//!
//! ```no_run
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), katharsis::Error> {
//!     let config_path = Path::new("katharsis.toml");
//!
//!     katharsis::init(config_path, false).await?;
//!     katharsis::build(config_path).await?;
//!
//!     Ok(())
//! }
//! ```

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod config;
mod error;
mod feed;
mod init;

pub use config::Config;
pub use error::Error;
pub use feed::build;
pub use init::init;
