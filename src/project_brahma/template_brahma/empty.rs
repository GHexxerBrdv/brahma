use super::git::init_git;
use crate::project_brahma::template_router::create_template;

const GITIGNORE_TEMPLATE: &str = "empty/gitignore";
const README_TEMPLATE: &str = "empty/README.md";

pub fn generate_empty(project_name: &str) -> std::io::Result<()> {
    init_git(project_name)?;
    create_template(
        GITIGNORE_TEMPLATE,
        &format!("{}/.gitignore", project_name),
        project_name,
    )?;
    create_template(
        README_TEMPLATE,
        &format!("{}/README.md", project_name),
        project_name,
    )?;
    Ok(())
}
