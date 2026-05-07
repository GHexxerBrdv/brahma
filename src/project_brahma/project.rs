use crate::project_brahma::template_router::{apply_template, install_dependencies};
use crate::project_brahma::template_selector::select_template;

use std::fs::create_dir_all;
use std::path::Path;

pub fn create_project(name: &str, template: bool) -> std::io::Result<()> {
    println!("Initializing process...");

    let path = Path::new(name);
    if path.exists() {
        println!("Project already exists");
        return Ok(());
    }

    create_dir_all(path)?;
    if template {
        let template = select_template();
        if let Some(template_name) = template {
            apply_template(&template_name, name)?;
            install_dependencies(&template_name, name)?;
        }
    } else {
        apply_template("None", name)?;
    }
    println!("Project {} created", name);
    Ok(())
}
