pub use anyhow::{Context, Result};
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrahmaError {
    #[error("Project already exists")]
    ProjectAlreadyExists,
    #[error("Command failed")]
    CommandFailed,
    #[error("Invalid project paths")]
    InvalidProjectPaths,
    #[error("Selection cancelled by user")]
    UserCancelled,
}
