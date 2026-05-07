use crate::project_brahma::commands::run_command;
use crate::project_brahma::template_router::create_template;

const EXPRESS_PACKAGE_JSON: &str = "express/package.json";
const EXPRESS_GITIGNORE: &str = "express/gitignore";
const EXPRESS_INDEX_JS: &str = "express/index.js";
const EXPRESS_README: &str = "express/README.md";

pub fn install_express_dependencies(project_name: &str) -> std::io::Result<()> {
    run_command("npm", &["install", "express", "nodemon"], project_name)
}

pub fn generate_express(project_name: &str) -> std::io::Result<()> {
    // Generate package.json
    create_template(
        EXPRESS_PACKAGE_JSON,
        &format!("{}/package.json", project_name),
        project_name,
    )?;

    // Generate src/index.js
    create_template(
        EXPRESS_INDEX_JS,
        &format!("{}/src/index.js", project_name),
        project_name,
    )?;

    // Generate .gitignore
    create_template(
        EXPRESS_GITIGNORE,
        &format!("{}/.gitignore", project_name),
        project_name,
    )?;

    // Generate README.md
    create_template(
        EXPRESS_README,
        &format!("{}/README.md", project_name),
        project_name,
    )?;

    Ok(())
}
