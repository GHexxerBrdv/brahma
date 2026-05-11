use crate::errors::{BrahmaError, Context, Result};
use include_dir::{Dir, include_dir};
use std::fs::{create_dir_all, write};

pub static SHARED_TEMPLATES_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/../../templates/shared");
pub static EXTENSIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../extensions");

fn create_template(template_path: &str, output_path: &str, project_name: &str) -> Result<()> {
    let file = SHARED_TEMPLATES_DIR
        .get_file(template_path)
        .or_else(|| EXTENSIONS_DIR.get_file(template_path))
        .context("Template not found")?;

    let content = file
        .contents_utf8()
        .context("Template is not valid UTF-8")?;

    let rendered = content.replace("{{project_name}}", project_name);
    let path = std::path::Path::new(output_path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent).context("Creating parent directory failed")?;
    }
    write(path, rendered).context("Writing to file failed")?;
    Ok(())
}

pub fn create_templates(
    template_paths: &[&str],
    output_paths: &[&str],
    project_name: &str,
) -> Result<()> {
    if template_paths.len() != output_paths.len() {
        return Err(BrahmaError::InvalidProjectPaths.into());
    }
    for (template_path, output_path) in template_paths.iter().zip(output_paths.iter()) {
        create_template(template_path, output_path, project_name)
            .context("Failed to create template")?;
    }
    Ok(())
}
