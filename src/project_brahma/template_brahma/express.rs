use crate::project_brahma::commands::run_command;
use crate::project_brahma::template_router::create_template;

const EXPRESS_PACKAGE_JSON: &str = "express/package.json";
const EXPRESS_GITIGNORE: &str = "express/gitignore";
const EXPRESS_README: &str = "express/README.md";
const EXPRESS_ENV: &str = "express/.env";
const EXPRESS_APP_JS: &str = "express/src/app.js";
const EXPRESS_SERVER_JS: &str = "express/src/server.js";
const EXPRESS_DB_JS: &str = "express/src/config/db.js";
const EXPRESS_USER_CONTROLLER: &str = "express/src/controllers/user.controller.js";
const EXPRESS_ERROR_MIDDLEWARE: &str = "express/src/middlewares/error.middleware.js";
const EXPRESS_USER_MODEL: &str = "express/src/models/user.model.js";
const EXPRESS_USER_ROUTES: &str = "express/src/routes/user.routes.js";
const EXPRESS_USER_SERVICE: &str = "express/src/services/user.service.js";
const EXPRESS_API_RESPONSE: &str = "express/src/utils/apiResponce.js";

pub fn install_express_dependencies(project_name: &str) -> std::io::Result<()> {
    run_command(
        "npm",
        &[
            "install", "express", "nodemon", "dotenv", "mongoose", "cors", "morgan",
        ],
        project_name,
    )
}

pub fn generate_express(project_name: &str) -> std::io::Result<()> {
    // Root files
    create_template(
        EXPRESS_PACKAGE_JSON,
        &format!("{}/package.json", project_name),
        project_name,
    )?;
    create_template(
        EXPRESS_GITIGNORE,
        &format!("{}/.gitignore", project_name),
        project_name,
    )?;
    create_template(
        EXPRESS_README,
        &format!("{}/README.md", project_name),
        project_name,
    )?;
    create_template(EXPRESS_ENV, &format!("{}/.env", project_name), project_name)?;

    // src/
    create_template(
        EXPRESS_APP_JS,
        &format!("{}/src/app.js", project_name),
        project_name,
    )?;
    create_template(
        EXPRESS_SERVER_JS,
        &format!("{}/src/server.js", project_name),
        project_name,
    )?;

    // src/config/
    create_template(
        EXPRESS_DB_JS,
        &format!("{}/src/config/db.js", project_name),
        project_name,
    )?;

    // src/controllers/
    create_template(
        EXPRESS_USER_CONTROLLER,
        &format!("{}/src/controllers/user.controller.js", project_name),
        project_name,
    )?;

    // src/middlewares/
    create_template(
        EXPRESS_ERROR_MIDDLEWARE,
        &format!("{}/src/middlewares/error.middleware.js", project_name),
        project_name,
    )?;

    // src/models/
    create_template(
        EXPRESS_USER_MODEL,
        &format!("{}/src/models/user.model.js", project_name),
        project_name,
    )?;

    // src/routes/
    create_template(
        EXPRESS_USER_ROUTES,
        &format!("{}/src/routes/user.routes.js", project_name),
        project_name,
    )?;

    // src/services/
    create_template(
        EXPRESS_USER_SERVICE,
        &format!("{}/src/services/user.service.js", project_name),
        project_name,
    )?;

    // src/utils/
    create_template(
        EXPRESS_API_RESPONSE,
        &format!("{}/src/utils/apiResponce.js", project_name),
        project_name,
    )?;

    Ok(())
}
