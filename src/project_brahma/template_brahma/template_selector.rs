use super::constants::*;
use super::types::{ProjectFlavors, Projects};
use crate::errors::{BrahmaError, Result};
use anyhow::Context;
use cliclack::select;

fn select_project() -> Result<Projects> {
    let project: String = select("Select the project")
        .item(
            EXPRESS_CATEGORY,
            EXPRESS_LABLE,
            "Fast, unopinionated, minimalist web framework",
        )
        .item(HONO_CATEGORY, HONO_LABLE, "Coming soon")
        .item(NEST_CATEGORY, NEST_LABLE, "Coming soon")
        .interact()
        .ok()
        .unwrap()
        .to_string();

    match project.as_str() {
        EXPRESS_CATEGORY => return Ok(Projects::Express),
        HONO_CATEGORY => return Ok(Projects::Hono),
        NEST_CATEGORY => return Ok(Projects::Nest),
        _ => return Ok(Projects::Empty),
    }
}

fn select_flavor(project: Projects) -> Result<ProjectFlavors> {
    match project {
        Projects::Express => {
            let flavor: String = select("Select express flavor")
                .item(EXPRESS_JS_FLAVOR, EXPRESS_JS_LABLE, "Standard JavaScript")
                .item(EXPRESS_TS_FLAVOR, EXPRESS_TS_LABLE, "TypeScript support")
                .interact()
                .ok()
                .unwrap()
                .to_string();

            return match flavor.as_str() {
                EXPRESS_JS_FLAVOR => Ok(ProjectFlavors::ExpressJs),
                EXPRESS_TS_FLAVOR => Ok(ProjectFlavors::ExpressTs),
                _ => unreachable!(),
            };
        }
        Projects::Hono => {
            println!("{HONO_LABLE} Coming soon");
            return Ok(ProjectFlavors::None);
        }
        Projects::Nest => {
            println!("{NEST_LABLE} Coming soon");
            return Ok(ProjectFlavors::None);
        }
        Projects::Empty => return Ok(ProjectFlavors::None),
    }
}

pub fn select_template() -> Result<ProjectFlavors> {
    let project = select_project().context("Failed to select project")?;
    let flavor = select_flavor(project).context("Failed to select flavor")?;
    Ok(flavor)
}
