use crate::error::app::Errors;
use anyhow::Result;
use inquire::Select;
use std::path::PathBuf;
use tokio::fs;

/// Generate a config file.
///
/// # Examples
///
/// ```no_run
/// use anyhow::Result;
/// use katharsis::cmd;
/// use std::path::PathBuf;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let path = PathBuf::from("katharsis.config.toml");
///
///     cmd::handle::init(&path).await?;
///
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// - When the file’s existence cannot be verified.
/// - When the file cannot be created.
/// - When the options cannot be displayed.
/// - When the file cannot be written to.
pub async fn init(path: &PathBuf) -> Result<(), Errors> {
    let config_bytes = include_bytes!("katharsis.config.toml");

    if path.try_exists()? {
        let options: Vec<&str> = vec!["Yes", "No"];
        let ans: &str = Select::new("A katharsis.config.toml already exists in the current directory. Do you want to overwrite it?", options).prompt()?;

        if ans == "yes" {
            fs::write(path, config_bytes).await?;
        }
    } else {
        fs::write(path, config_bytes).await?;
    }

    Ok(())
}
