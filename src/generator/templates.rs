use include_dir::{include_dir, Dir};
use std::fs;

static TEMPLATES_DIR: Dir<'_> = include_dir!("templates");

pub fn generate_all(project_name: &str) -> std::io::Result<()> {
    // Generate package.json
    write_template(
        "express/package.json",
        &format!("{}/package.json", project_name),
        project_name,
    )?;

    // Generate index.js
    write_template(
        "express/index.js",
        &format!("{}/src/index.js", project_name),
        project_name,
    )?;

    // Generate .gitignore
    write_template(
        "express/gitignore",
        &format!("{}/.gitignore", project_name),
        project_name,
    )?;

    Ok(())
}

fn write_template(template_path: &str, output_path: &str, project_name: &str) -> std::io::Result<()> {
    let file = TEMPLATES_DIR
        .get_file(template_path)
        .ok_or_else(|| {
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
