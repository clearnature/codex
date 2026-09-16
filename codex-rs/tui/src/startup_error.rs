use codex_i18n::current;
use codex_i18n::tr_with;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct LocalStateDbStartupError {
    database_path: PathBuf,
    detail: String,
}

/// thiserror's `#[error("...")]` only accepts literals, so this type is rendered
/// by hand and routed through the dictionary instead.
impl std::fmt::Display for LocalStateDbStartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            tr_with(
                current(),
                "failed to initialize sqlite local db at {0}: {1}",
                &[&self.database_path.display().to_string(), &self.detail],
            )
        )
    }
}

impl std::error::Error for LocalStateDbStartupError {}

impl LocalStateDbStartupError {
    pub fn new(database_path: PathBuf, detail: String) -> Self {
        Self {
            database_path,
            detail,
        }
    }

    pub fn database_path(&self) -> &Path {
        self.database_path.as_path()
    }

    pub fn state_db_path(&self) -> &Path {
        self.database_path()
    }

    pub fn detail(&self) -> &str {
        self.detail.as_str()
    }
}
