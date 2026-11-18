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
    /// Specify the path to the configuration file
    #[arg(
        short,
        long,
        env = "KATHARSIS_CONFIG",
        global = true,
        default_value = "katharsis.toml"
    )]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a default configuration file
    Init {
        /// Skip interactive confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Build the RSS feed from the configuration file
    Build,
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

    match cli.command {
        Commands::Init { yes } => katharsis::init(&cli.config, yes).await?,
        Commands::Build => {
            katharsis::build(&cli.config).await?;
        }
    }

    Ok(())
}
