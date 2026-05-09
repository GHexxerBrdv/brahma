use crate::errors::{BrahmaError, Context, Result};
use std::process::Command;

pub fn run_command(cmd: &str, args: &[&str], dir: &str) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()
        .context("Command failed")?;

    if !status.success() {
        return Err(BrahmaError::CommandFailed.into());
    }
    Ok(())
}
