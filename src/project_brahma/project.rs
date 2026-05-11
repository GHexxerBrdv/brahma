use super::template_brahma::template_router::{install_dependencies, route_template};
use super::template_brahma::template_selector::select_template;
use super::template_brahma::types::ProjectFlavors;
use crate::errors::{BrahmaError, Context, Result};
use std::fs::create_dir_all;
use std::path::Path;

pub fn create_project(name: &str, template: bool) -> Result<()> {
    let path = Path::new(name);
    if path.exists() {
        return Err(BrahmaError::ProjectAlreadyExists.into());
    }

    let project_type = if template {
        select_template().context("Failed to select template")?
    } else {
        ProjectFlavors::None
    };

    let spinner = cliclack::spinner();

    spinner.start("Initializing process...");

    create_dir_all(path).context("Failed to create project directory")?;

    let template_str = project_type.as_str();
    route_template(template_str, name).context("Failed to route template")?;

    spinner.set_message("Installing dependencies...");
    install_dependencies(template_str, name).context("Failed to install dependencies")?;

    spinner.stop(format!("Project {} created successfully!", name));

    Ok(())
}
