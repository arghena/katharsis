//! CLI entry point for Katharsis.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{
    env,
    ffi::OsStr,
    io::{self, IsTerminal},
    path::PathBuf,
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt::writer::MakeWriterExt};

#[derive(Debug, Parser, PartialEq, Eq)]
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

#[derive(Debug, Subcommand, PartialEq, Eq)]
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
    let cli = Cli::parse();
    init_tracing();
    run(&cli).await
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn init_tracing() {
    let writer = io::stderr.with_max_level(Level::WARN).or_else(io::stdout);
    let ansi = should_enable_ansi(
        io::stdout().is_terminal() && io::stderr().is_terminal(),
        env::var_os("NO_COLOR").as_deref(),
        env::var_os("TERM").as_deref(),
    );
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(writer)
        .with_ansi(ansi)
        .with_target(false)
        .without_time()
        .init();
}

fn should_enable_ansi(is_terminal: bool, no_color: Option<&OsStr>, term: Option<&OsStr>) -> bool {
    is_terminal && no_color.is_none_or(OsStr::is_empty) && term.is_none_or(|t| t != "dumb")
}

async fn run(cli: &Cli) -> Result<()> {
    match cli.command {
        Commands::Init { yes } => katharsis::init(&cli.config, yes).await?,
        Commands::Build => {
            katharsis::build(&cli.config).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use tempfile::tempdir;
    use tokio::fs;
    use tokio_test::assert_ok;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_cli_commands() {
        let init = assert_ok!(Cli::try_parse_from(["katharsis", "init", "--yes"]));
        assert_eq!(
            init,
            Cli {
                config: PathBuf::from("katharsis.toml"),
                command: Commands::Init { yes: true },
            }
        );

        let build = assert_ok!(Cli::try_parse_from([
            "katharsis",
            "--config",
            "custom.toml",
            "build",
        ]));
        assert_eq!(
            build,
            Cli {
                config: PathBuf::from("custom.toml"),
                command: Commands::Build,
            }
        );
    }

    #[test]
    fn ansi_detection_respects_terminal_and_env() {
        let xterm = Some(OsStr::new("xterm-256color"));
        let dumb = Some(OsStr::new("dumb"));
        let empty = Some(OsStr::new(""));
        let enabled = Some(OsStr::new("1"));

        assert!(should_enable_ansi(true, None, None));
        assert!(should_enable_ansi(true, None, xterm));
        assert!(should_enable_ansi(true, empty, xterm));

        assert!(!should_enable_ansi(false, None, xterm));
        assert!(!should_enable_ansi(true, enabled, xterm));
        assert!(!should_enable_ansi(true, None, dumb));
    }

    #[tokio::test]
    async fn run_executes_init_and_build() {
        let dir = assert_ok!(tempdir());
        let config_path = dir.path().join("katharsis.toml");
        let output_path = dir.path().join("rss.xml");

        assert_ok!(
            run(&Cli {
                config: config_path.clone(),
                command: Commands::Init { yes: true },
            })
            .await
        );
        assert!(assert_ok!(config_path.try_exists()));

        let template = assert_ok!(fs::read_to_string(&config_path).await);
        let output_str = output_path.to_string_lossy().replace('\\', "/");
        let updated = template.replace(
            r#"output = "rss.xml""#,
            &format!(r#"output = "{output_str}""#),
        );
        assert_ok!(fs::write(&config_path, updated).await);

        assert_ok!(
            run(&Cli {
                config: config_path,
                command: Commands::Build,
            })
            .await
        );
        assert!(assert_ok!(output_path.try_exists()));
    }
}
