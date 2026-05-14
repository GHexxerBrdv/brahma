use super::constants::*;
use crate::errors::{Context, Result};
use crate::project_brahma::commands::run_command_guarded;
use crate::project_brahma::project_template_brahma::git::init_git;
use crate::project_brahma::template_brahma::template_creator::create_template;

pub fn install_express_ts_dependencies(project_name: &str) -> Result<()> {
    run_command_guarded(
        NPM,
        &[INSTALL, EXPRESS, DOTENV, MONGOOSE, CORS, MORGAN],
        project_name,
    )?;

    run_command_guarded(
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

pub fn generate_express_ts(project_name: &str, git: bool) -> Result<()> {
    create_template("express/express_ts", project_name)
        .context("Failed creating express-ts project")?;

    if git {
        init_git(project_name).context("Git initialization failed")?;
    }
    Ok(())
}
