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

fn select_flavor(project: Projects) -> Result<(ProjectFlavors, bool)> {
    match project {
        Projects::Express => {
            let flavor: &str = select("Select express flavor")
                .item(EXPRESS_JS_FLAVOR, EXPRESS_JS_LABEL, "Standard JavaScript")
                .item(EXPRESS_TS_FLAVOR, EXPRESS_TS_LABEL, "TypeScript support")
                .interact()
                .map_err(|_| BrahmaError::UserAborted)?;

            let git: &str = select("Do you want to initialize a git repository?")
                .item("yes", "Yes", "Initialize a git repository")
                .item("no", "No", "Do not initialize a git repository")
                .interact()
                .map_err(|_| BrahmaError::UserAborted)?;

            match flavor {
                EXPRESS_JS_FLAVOR => Ok((ProjectFlavors::ExpressJs, git == "yes")),
                EXPRESS_TS_FLAVOR => Ok((ProjectFlavors::ExpressTs, git == "yes")),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub fn select_template() -> Result<(ProjectFlavors, bool)> {
    let project = select_project().context("Failed to select project")?;
    let (flavor, git) = select_flavor(project).context("Failed to select flavor")?;
    Ok((flavor, git))
}
