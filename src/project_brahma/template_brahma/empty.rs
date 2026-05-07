use super::git::init_git;
use crate::project_brahma::template_router::create_template;

pub fn generate_empty(project_name: &str) -> std::io::Result<()> {
    init_git(project_name)?;
    create_template(
        "empty/gitignore",
        &format!("{}/.gitignore", project_name),
        project_name,
    )?;
    create_template(
        "empty/README.md",
        &format!("{}/README.md", project_name),
        project_name,
    )?;
    Ok(())
}
