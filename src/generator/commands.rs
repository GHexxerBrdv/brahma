use std::process::Command;

pub fn run_command(cmd: &str, args: &[&str], dir: &str) -> std::io::Result<()> {
    let status = Command::new(cmd).args(args).current_dir(dir).status()?;

    if !status.success() {
        println!("Command Failed: {} {:?}", cmd, args);
    }

    Ok(())
}
