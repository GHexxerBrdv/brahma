use super::constants::*;
use crate::errors::{Context, Result};
use crate::project_brahma::commands::run_command;
use crate::project_brahma::project_template_brahma::git::init_git;
use crate::project_brahma::template_brahma::template_creator::create_templates;

pub fn install_express_dependencies(project_name: &str) -> Result<()> {
    run_command(
        NPM,
        &[INSTALL, EXPRESS, NODMON, DOTENV, MONGOOSE, CORS, MORGAN],
        project_name,
    )
}

pub fn generate_express(project_name: &str) -> Result<()> {
    let template_paths = vec![
        EXPRESS_PACKAGE_JSON,
        EXPRESS_GITIGNORE,
        EXPRESS_README,
        EXPRESS_ENV,
        EXPRESS_APP_JS,
        EXPRESS_SERVER_JS,
        EXPRESS_DB_JS,
        EXPRESS_USER_CONTROLLER,
        EXPRESS_ERROR_MIDDLEWARE,
        EXPRESS_USER_MODEL,
        EXPRESS_USER_ROUTES,
        EXPRESS_USER_SERVICE,
        EXPRESS_API_RESPONSE,
    ];

    let output_paths: [&str; 13] = [
        &format!("{}/package.json", project_name),
        &format!("{}/.gitignore", project_name),
        &format!("{}/README.md", project_name),
        &format!("{}/.env", project_name),
        &format!("{}/src/app.js", project_name),
        &format!("{}/src/server.js", project_name),
        &format!("{}/src/config/db.js", project_name),
        &format!("{}/src/controllers/user.controller.js", project_name),
        &format!("{}/src/middlewares/error.middleware.js", project_name),
        &format!("{}/src/models/user.model.js", project_name),
        &format!("{}/src/routes/user.routes.js", project_name),
        &format!("{}/src/services/user.service.js", project_name),
        &format!("{}/src/utils/apiResponce.js", project_name),
    ];

    create_templates(&template_paths, &output_paths, project_name)
        .context("Failed creating express project")?;

    println!("> Initializing git...");
    init_git(project_name).context("Git initialization failed")?;

    Ok(())
}
