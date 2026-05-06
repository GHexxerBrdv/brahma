use std::fs::{read_to_string, write};

pub const EXPRESS_PACKAGE_JSON_PATH: &str = "src/templates/express/package.json";
pub const EXPRESS_INDEX_JS_PATH: &str = "src/templates/express/index.js";

pub fn generate_all(project_name: &str) -> std::io::Result<()> {
    create_from_template(
        EXPRESS_PACKAGE_JSON_PATH,
        &format!("{}/package.json", project_name),
        project_name,
    )?;

    // index.js
    create_from_template(
        EXPRESS_INDEX_JS_PATH,
        &format!("{}/src/index.js", project_name),
        project_name,
    )?;

    Ok(())
}

fn render_template(path: &str, name: &str) -> String {
    let content = read_to_string(path).expect("Failed to read template");
    content.replace("{{project_name}}", name)
}

fn create_from_template(template_path: &str, output_path: &str, name: &str) -> std::io::Result<()> {
    let rendered = render_template(template_path, name);
    write(output_path, rendered)?;
    Ok(())
}
