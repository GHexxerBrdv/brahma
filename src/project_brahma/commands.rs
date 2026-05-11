use crate::errors::{BrahmaError, Context, Result};
use std::process::{Command, Stdio};

pub fn run_command(cmd: &str, args: &[&str], dir: &str) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Command failed")?;

    if !status.success() {
        return Err(BrahmaError::CommandFailed.into());
    }
    Ok(())
}
