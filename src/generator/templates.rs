use include_dir::{Dir, include_dir};
use std::fs;

static TEMPLATES_DIR: Dir<'_> = include_dir!("templates");

const EXPRESS_PACKAGE_JSON: &str = "express/package.json";
const EXPRESS_GITIGNORE: &str = "express/gitignore";
const EXPRESS_INDEX_JS: &str = "express/index.js";

pub fn generate_all(project_name: &str) -> std::io::Result<()> {
    create_template(
        EXPRESS_PACKAGE_JSON,
        &format!("{}/package.json", project_name),
        project_name,
    )?;

    create_template(
        EXPRESS_INDEX_JS,
        &format!("{}/src/index.js", project_name),
        project_name,
    )?;

    create_template(
        EXPRESS_GITIGNORE,
        &format!("{}/.gitignore", project_name),
        project_name,
    )?;

    Ok(())
}

fn create_template(
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
    fs::write(output_path, rendered)?;

    Ok(())
}
