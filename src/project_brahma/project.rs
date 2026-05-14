use super::template_brahma::template_router::{install_dependencies, route_template};
use super::template_brahma::template_selector::select_template;
use super::template_brahma::types::ProjectFlavors;
use crate::errors::{BrahmaError, Context, Result};
use cliclack::spinner;
use std::path::Path;

pub fn create_project(name: &str, template: bool) -> Result<()> {
    let path = Path::new(name);
    if path.exists() {
        return Err(BrahmaError::ProjectAlreadyExists.into());
    }

    let (project_type, git) = if template {
        select_template().context("Failed to select template")?
    } else {
        (ProjectFlavors::None, false)
    };

    let spinner = spinner();

    spinner.start("Initializing process...");

    // create_dir_all(path).context("Failed to create project directory")?; //>/issue:- move it to the template creation

    let template_str = project_type.as_str();
    route_template(template_str, name, git).context("Failed to route template")?;

    spinner.set_message("Installing dependencies...");
    if project_type != ProjectFlavors::None {
        install_dependencies(template_str, name).context("Failed to install dependencies")?;
    }

    spinner.stop(format!("Project {} created successfully!", name));

    Ok(())
}
