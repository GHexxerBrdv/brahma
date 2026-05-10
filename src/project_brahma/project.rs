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

    create_dir_all(path).context("Failed to create directory")?;

    let project_flavor = if template {
        select_template().context("Failed to select template")?
    } else {
        ProjectFlavors::None
    };

    let project = project_flavor.as_str();
    route_template(project, name).context("Failed to route template")?;
    install_dependencies(project, name).context("Failed to install dependencies")?;

    println!("Project {} created successfully!", name);

    Ok(())
}
