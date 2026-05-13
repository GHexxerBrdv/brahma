pub use anyhow::{Context, Result};
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrahmaError {
    #[error("Project already exists")]
    ProjectAlreadyExists,
    #[error("Command failed")]
    CommandFailed,
    // #[error("Invalid project paths")]
    // InvalidProjectPaths,
    #[error("User aborted")]
    UserAborted,
    #[error("Install failed")]
    InstallFailed(String),
    #[error("Dependency missing")]
    DependencyMissing(String),
}
