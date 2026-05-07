use crate::project_brahma::commands::run_command;
use crate::project_brahma::template_router::apply_template;

pub fn init_git(project_name: &str) -> std::io::Result<()> {
    run_command("git", &["init"], project_name)?;
    apply_template("None", project_name)?;
    Ok(())
}
