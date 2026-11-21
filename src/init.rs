use crate::{config::DEFAULT_TEMPLATE, error::Error};
use inquire::Confirm;
use std::path::Path;
use tokio::fs;
use tracing::info;

/// Generate a default configuration file at `path`.
///
/// If `yes` is `true`, any existing file at `path` will be overwritten without
/// displaying an interactive confirmation prompt.
///
/// # Examples
///
/// ```no_run
/// use anyhow::Result;
/// use std::path::Path;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     katharsis::init(Path::new("katharsis.toml"), false).await?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// - When the file's existence cannot be verified.
/// - When the interactive prompt fails to display.
/// - When the file cannot be created or written to.
pub async fn init(path: &Path, yes: bool) -> Result<(), Error> {
    init_with_prompt(path, yes, prompt_overwrite).await
}

async fn init_with_prompt<F>(path: &Path, yes: bool, confirm_overwrite: F) -> Result<(), Error>
where
    F: FnOnce(&Path) -> Result<bool, Error>,
{
    if !yes && fs::try_exists(path).await? && !confirm_overwrite(path)? {
        info!(path = %path.display(), "skipped overwriting existing configuration");
        return Ok(());
    }

    fs::write(path, DEFAULT_TEMPLATE).await?;
    info!(path = %path.display(), "initialized configuration file");

    Ok(())
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn prompt_overwrite(path: &Path) -> Result<bool, Error> {
    let message = format!(
        "`{}` already exists. Do you want to overwrite it?",
        path.display()
    );
    Ok(Confirm::new(&message).with_default(false).prompt()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio_test::assert_ok;

    #[tokio::test]
    async fn init_creates_new_file() {
        let dir = assert_ok!(tempdir());
        let path = dir.path().join("katharsis.toml");

        assert_ok!(init(&path, false).await);

        let content = assert_ok!(fs::read_to_string(&path).await);
        assert_eq!(content, DEFAULT_TEMPLATE);
    }

    #[tokio::test]
    async fn init_overwrites_without_prompt_when_yes_is_true() {
        let dir = assert_ok!(tempdir());
        let path = dir.path().join("katharsis.toml");
        assert_ok!(fs::write(&path, "old_content").await);

        assert_ok!(init(&path, true).await);

        let content = assert_ok!(fs::read_to_string(&path).await);
        assert_eq!(content, DEFAULT_TEMPLATE);
    }

    #[tokio::test]
    async fn init_overwrites_when_confirmed() {
        let dir = assert_ok!(tempdir());
        let path = dir.path().join("katharsis.toml");
        assert_ok!(fs::write(&path, "old_content").await);

        assert_ok!(init_with_prompt(&path, false, |_| Ok(true)).await);

        let content = assert_ok!(fs::read_to_string(&path).await);
        assert_eq!(content, DEFAULT_TEMPLATE);
    }

    #[tokio::test]
    async fn init_preserves_existing_file_when_declined() {
        let dir = assert_ok!(tempdir());
        let path = dir.path().join("katharsis.toml");
        assert_ok!(fs::write(&path, "custom_content").await);

        assert_ok!(init_with_prompt(&path, false, |_| Ok(false)).await);

        let content = assert_ok!(fs::read_to_string(&path).await);
        assert_eq!(content, "custom_content");
    }
}
