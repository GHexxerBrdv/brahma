use crate::project_brahma::commands::run_command;

const COMMAND: &str = "git";
const ARGS: &[&str] = &["init"];

pub fn init_git(project_name: &str) -> std::io::Result<()> {
    run_command(COMMAND, ARGS, project_name)?;
    Ok(())
}
