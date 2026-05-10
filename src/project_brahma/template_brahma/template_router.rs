use super::constants::*;
use crate::errors::{Context, Result};
use crate::project_brahma::project_template_brahma::empty::empty::generate_empty;
use crate::project_brahma::project_template_brahma::express::express_js::express_js::{
    generate_express_js, install_express_js_dependencies,
};
use crate::project_brahma::project_template_brahma::express::express_ts::express_ts::{
    generate_express_ts, install_express_ts_dependencies,
};

pub fn install_dependencies(template: &str, project: &str) -> Result<()> {
    match template {
        NONE => {
            println!("No dependencies to install");
            return Ok(());
        }
        EXPRESS_JS_FLAVOR => {
            println!("> Installing express dependencies...");
            install_express_js_dependencies(project)
                .context("Installing express dependencies failed")?
        }
        EXPRESS_TS_FLAVOR => {
            println!("> Installing express-ts dependencies...");
            install_express_ts_dependencies(project)
                .context("Installing express-ts dependencies failed")?
        }

        _ => {}
    }
    Ok(())
}

pub fn route_template(template: &str, project: &str) -> Result<()> {
    match template {
        NONE => {
            println!("> Creating empty project...");
            generate_empty(project).context("Failed to create empty project")?;
        }
        EXPRESS_JS_FLAVOR => {
            println!("> Creating express-js project...");
            generate_express_js(project).context("Failed to generate express project")?;
        }
        EXPRESS_TS_FLAVOR => {
            println!("> Creating express-ts project...");
            generate_express_ts(project).context("Failed to generate express-ts project")?;
        }
        _ => {} // beta
    }
    Ok(())
}
