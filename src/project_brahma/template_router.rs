use include_dir::{Dir, include_dir};
use std::fs;

use crate::project_brahma::template_brahma::empty::generate_empty;
use crate::project_brahma::template_brahma::express::{
    generate_express, install_express_dependencies,
};

pub static TEMPLATES_DIR: Dir<'_> = include_dir!("templates");

pub fn create_template(
    template_path: &str,
    output_path: &str,
    project_name: &str,
) -> std::io::Result<()> {
    let file = TEMPLATES_DIR.get_file(template_path).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Template not found: {}", template_path),
        )
    })?;

    let content = file.contents_utf8().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Template is not valid UTF-8: {}", template_path),
        )
    })?;

    let rendered = content.replace("{{project_name}}", project_name);
    let path = std::path::Path::new(output_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    fs::write(path, rendered)?;
    Ok(())
}

pub fn install_dependencies(template_name: &str, project_name: &str) -> std::io::Result<()> {
    if template_name == "express" {
        install_express_dependencies(project_name)?
    }
    Ok(())
}

pub fn apply_template(template_name: &str, project_name: &str) -> std::io::Result<()> {
    if template_name == "express" {
        generate_express(project_name)?;
    } else {
        generate_empty(project_name)?;
    }
    Ok(())
}
