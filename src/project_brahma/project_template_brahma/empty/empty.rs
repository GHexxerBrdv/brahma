use crate::errors::{Context, Result};
use crate::project_brahma::project_template_brahma::git::init_git;
use crate::project_brahma::template_brahma::template_creator::create_template;

pub fn generate_empty(project_name: &str, git: bool) -> Result<()> {
    create_template("empty", project_name).context("Failed creating empty project")?;

    if git {
        init_git(project_name).context("Git initialization failed")?;
    }

    Ok(())
}
