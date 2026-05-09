use super::template_router::{install_dependencies, route_template};
use super::template_selector::select_template;
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
    if template {
        let template = select_template();
        if let Some(template_name) = template {
            route_template(&template_name, name).context("Failed")?;
            install_dependencies(&template_name, name).context("Failed install dependencies")?;
        }
    } else {
        route_template("Empty", name).context("Failed")?;
    }
    println!("Project {} created", name);
    Ok(())
}
