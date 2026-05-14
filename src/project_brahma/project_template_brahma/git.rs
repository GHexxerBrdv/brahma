use crate::errors::{Context, Result};
use crate::project_brahma::commands::run_command_guarded;

const GIT: &str = "git";
const ARGS: &[&str] = &["init"];

pub fn init_git(project_name: &str) -> Result<()> {
    run_command_guarded(GIT, ARGS, project_name).context("Command execution failed")?;
    Ok(())
}
