use super::template_brahma::template_router::{install_dependencies, route_template};
use super::template_brahma::template_selector::select_template;
use super::template_brahma::types::ProjectType;
use crate::errors::{BrahmaError, Context, Result};
use std::fs::create_dir_all;
use std::path::Path;

pub fn create_project(name: &str, template: bool) -> Result<()> {
    println!("Initializing process...");

    let path = Path::new(name);
    if path.exists() {
        return Err(BrahmaError::ProjectAlreadyExists.into());
    }

    create_dir_all(path)?;

    let project_type = if template {
        select_template()?
    } else {
        ProjectType::Empty
    };

    let template_str = project_type.as_str();
    route_template(template_str, name).context("Failed to route template")?;
    install_dependencies(template_str, name).context("Failed to install dependencies")?;

    println!("Project {} created successfully!", name);
    Ok(())
}
