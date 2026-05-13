use crate::errors::{BrahmaError, Context, Result};
use cliclack::{confirm, note, spinner};
use std::process::{Command, Stdio};
use which::which;

pub fn run_command_guarded(cmd: &str, args: &[&str], dir: &str) -> Result<()> {
    if !is_installed(cmd) {
        install_binary(cmd)?
    }

    execute_command(cmd, args, Some(dir))
}

fn execute_command(cmd: &str, args: &[&str], dir: Option<&str>) -> Result<()> {
    let mut command = Command::new(cmd);

    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit()); // can make it inherit to hide warnings and errors

    if let Some(dir) = dir {
        command.current_dir(dir);
    }

    let status = command.status().context("Command execution failed")?;

    if !status.success() {
        return Err(BrahmaError::CommandFailed.into());
    }
    Ok(())
}

fn is_installed(binary: &str) -> bool {
    which(binary).is_ok()
}

fn install_binary(binary: &str) -> Result<()> {
    if !prompt_install(binary) {
        return Err(BrahmaError::UserAborted.into());
    }

    let (cmd, args) =
        get_install_command(binary).ok_or_else(|| BrahmaError::DependencyMissing(binary.into()))?;

    let spinner = spinner();

    spinner.start(format!("Installing {}...", binary));

    match execute_command(cmd, &args, None) {
        Ok(_) => {
            spinner.stop(format!("{binary} installed successfully"));
            Ok(())
        }
        Err(_) => {
            spinner.stop(format!("Failed to install {binary}"));
            Err(BrahmaError::InstallFailed(binary.into()).into())
        }
    }
}

fn get_install_command(binary: &str) -> Option<(&'static str, Vec<&'static str>)> {
    platform_install_command(binary)
}

#[cfg(target_os = "linux")]
fn platform_install_command(binary: &str) -> Option<(&'static str, Vec<&'static str>)> {
    match binary {
        "git" => install_linux_binary("git"),
        _ => None,
    }
}

#[cfg(target_os = "macos")]
fn platform_install_command(binary: &str) -> Option<(&'static str, Vec<&'static str>)> {
    if !is_installed("brew") {
        return None;
    }
    match binary {
        "git" => Some(("brew", vec!["install", "git"])),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn install_linux_binary(package: &'static str) -> Option<(&'static str, Vec<&'static str>)> {
    if is_installed("apt") {
        Some(("sudo", vec!["apt", "install", "-y", package]))
    } else if is_installed("dnf") {
        Some(("sudo", vec!["dnf", "install", "-y", package]))
    } else if is_installed("yum") {
        Some(("sudo", vec!["yum", "install", "-y", package]))
    } else if is_installed("pacman") {
        Some(("sudo", vec!["pacman", "-S", "--noconfirm", package]))
    } else if is_installed("zypper") {
        Some(("sudo", vec!["zypper", "install", "-y", package]))
    } else {
        None
    }
}

fn prompt_install(binary: &str) -> bool {
    note(
        "Missing Dependency",
        format!(
            "The tool '{}' is required for this operation, but not installed in your system.",
            binary
        ),
    )
    .ok();
    confirm(format!("Install {binary} now?"))
        .interact()
        .unwrap_or(false)
}
