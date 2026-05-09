use super::template_brahma::empty::empty::generate_empty;
use super::template_brahma::express::express::{generate_express, install_express_dependencies};
use crate::errors::{Context, Result};

pub fn install_dependencies(template_name: &str, project_name: &str) -> Result<()> {
    if template_name == "express" {
        println!("> Installing express dependencies...");
        install_express_dependencies(project_name)
            .context("Installing express dependencies failed")?
    }
    Ok(())
}

pub fn route_template(template_name: &str, project_name: &str) -> Result<()> {
    if template_name == "express" {
        println!("> Creating express project...");
        generate_express(project_name).context("Failed to generate express project")?;
    } else {
        println!("> Creating empty project...");
        generate_empty(project_name).context("Failed to create empty project")?;
    }
    Ok(())
}
