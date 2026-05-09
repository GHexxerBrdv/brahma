use super::constants::*;
use crate::errors::{Context, Result};
use crate::project_brahma::template_brahma::git::init_git;
use crate::project_brahma::template_creator::create_templates;

pub fn generate_empty(project_name: &str) -> Result<()> {
    let template_paths = vec![GITIGNORE_TEMPLATE, README_TEMPLATE];
    let output_paths: [&str; 2] = [
        &format!("{}/.gitignore", project_name),
        &format!("{}/README.md", project_name),
    ];

    create_templates(&template_paths, &output_paths, project_name)
        .context("Failed creating empty project")?;

    println!("> Initializing git...");
    init_git(project_name).context("Git initialization failed")?;

    Ok(())
}
