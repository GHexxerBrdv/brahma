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
        EXPRESS_JS_FLAVOR => {
            install_express_js_dependencies(project)
                .context("Installing express dependencies failed")?;

            cliclack::note("Next steps", format!("cd {}\nnpm run dev", project))?;
        }
        EXPRESS_TS_FLAVOR => {
            install_express_ts_dependencies(project)
                .context("Installing express-ts dependencies failed")?;

            cliclack::note("Next steps", format!("cd {}\nnpm run dev", project))?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub fn route_template(template: &str, project: &str) -> Result<()> {
    match template {
        NONE => {
            generate_empty(project).context("Failed to create empty project")?;
        }
        EXPRESS_JS_FLAVOR => {
            generate_express_js(project).context("Failed to generate express project")?;
        }
        EXPRESS_TS_FLAVOR => {
            generate_express_ts(project).context("Failed to generate express-ts project")?;
        }
        _ => unreachable!(),
    }
    Ok(())
}
