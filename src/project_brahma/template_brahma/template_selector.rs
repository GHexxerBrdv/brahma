use super::constants::*;
use super::types::{ProjectFlavors, Projects};
use crate::errors::{BrahmaError, Context, Result};
use cliclack::select;

fn select_project() -> Result<Projects> {
    let project: &str = select("Select the project")
        .item(
            EXPRESS_CATEGORY,
            EXPRESS_LABEL,
            "Fast, unopinionated, minimalist web framework",
        )
        .item(HONO_CATEGORY, HONO_LABEL, "Coming soon")
        .item(NEST_CATEGORY, NEST_LABEL, "Coming soon")
        .interact()
        .map_err(|_| BrahmaError::UserAborted)?;

    match project {
        EXPRESS_CATEGORY => Ok(Projects::Express),
        HONO_CATEGORY => Ok(Projects::Hono),
        NEST_CATEGORY => Ok(Projects::Nest),
        _ => unreachable!(),
    }
}

fn select_flavor(project: Projects) -> Result<ProjectFlavors> {
    match project {
        Projects::Express => {
            let flavor: &str = select("Select express flavor")
                .item(EXPRESS_JS_FLAVOR, EXPRESS_JS_LABEL, "Standard JavaScript")
                .item(
                    EXPRESS_JS_NO_GIT_FLAVOR,
                    EXPRESS_JS_NO_GIT_LABEL,
                    "Standard JavaScript (no git)",
                )
                .item(EXPRESS_TS_FLAVOR, EXPRESS_TS_LABEL, "TypeScript support")
                .item(
                    EXPRESS_TS_NO_GIT_FLAVOR,
                    EXPRESS_TS_NO_GIT_LABEL,
                    "TypeScript support (no git)",
                )
                .interact()
                .map_err(|_| BrahmaError::UserAborted)?;

            match flavor {
                EXPRESS_JS_FLAVOR => Ok(ProjectFlavors::ExpressJs),
                EXPRESS_JS_NO_GIT_FLAVOR => Ok(ProjectFlavors::ExpressJsNoGit),
                EXPRESS_TS_FLAVOR => Ok(ProjectFlavors::ExpressTs),
                EXPRESS_TS_NO_GIT_FLAVOR => Ok(ProjectFlavors::ExpressTsNoGit),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub fn select_template() -> Result<ProjectFlavors> {
    let project = select_project().context("Failed to select project")?;
    let flavor = select_flavor(project).context("Failed to select flavor")?;
    Ok(flavor)
}
