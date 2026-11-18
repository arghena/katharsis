//! CLI entry point for Katharsis.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{io, path::PathBuf};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt::writer::MakeWriterExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Use a katharsis.toml file as the config
    #[arg(env = "KATHARSIS_CONFIG", short, long, value_name = "CONFIG_FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a katharsis.toml file in the current directory
    Init {
        /// Overwrite existing configuration file without confirmation
        #[arg(short, long)]
        yes: bool,
    },
}

#[tokio::main]
#[cfg_attr(coverage_nightly, coverage(off))]
async fn main() -> Result<()> {
    let writer = io::stderr.with_max_level(Level::WARN).or_else(io::stdout);

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(writer)
        .with_target(false)
        .without_time()
        .init();

    let cli = Cli::parse();
    let path = cli
        .config
        .unwrap_or_else(|| PathBuf::from("katharsis.toml"));

    if let Some(Commands::Init { yes }) = cli.command {
        katharsis::init(&path, yes).await?;
    } else {
        katharsis::build(&path).await?;
    }

    Ok(())
}
