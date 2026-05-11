use super::constants::*;
use crate::errors::{Context, Result};
use crate::project_brahma::commands::run_command;
use crate::project_brahma::project_template_brahma::git::init_git;
use crate::project_brahma::template_brahma::template_creator::create_templates;

pub fn install_express_ts_dependencies(project_name: &str) -> Result<()> {
    println!("> Installing core dependencies...");
    run_command(
        NPM,
        &[INSTALL, EXPRESS, DOTENV, MONGOOSE, CORS, MORGAN],
        project_name,
    )?;

    println!("> Installing dev dependencies...");
    run_command(
        NPM,
        &[
            INSTALL,
            SAVE_DEV,
            TYPESCRIPT,
            TS_NODE_DEV,
            TYPES_EXPRESS,
            TYPES_NODE,
            TYPES_CORS,
            TYPES_MORGAN,
        ],
        project_name,
    )
}

pub fn generate_express_ts(project_name: &str) -> Result<()> {
    let template_paths = vec![
        EXPRESS_TS_PACKAGE_JSON,
        EXPRESS_TS_TSCONFIG,
        EXPRESS_TS_GITIGNORE,
        EXPRESS_TS_README,
        EXPRESS_TS_ENV,
        EXPRESS_TS_APP,
        EXPRESS_TS_SERVER,
        EXPRESS_TS_DB,
    ];

    let output_paths: [&str; 8] = [
        &format!("{}/package.json", project_name),
        &format!("{}/tsconfig.json", project_name),
        &format!("{}/.gitignore", project_name),
        &format!("{}/README.md", project_name),
        &format!("{}/.env", project_name),
        &format!("{}/src/app.ts", project_name),
        &format!("{}/src/server.ts", project_name),
        &format!("{}/src/config/db.ts", project_name),
    ];

    create_templates(&template_paths, &output_paths, project_name)
        .context("Failed creating express-ts project")?;

    println!("> Initializing git...");
    init_git(project_name).context("Git initialization failed")?;

    Ok(())
}
