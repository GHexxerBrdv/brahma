use super::types::ProjectType;
use crate::errors::{BrahmaError, Result};
use cliclack::select;

const EXPRESS_CATEGORY: &str = "express";
const HONO_CATEGORY: &str = "hono";
const NEST_CATEGORY: &str = "nest";

pub fn select_template() -> Result<ProjectType> {
    let category: &str = select("Select the project type")
        .item(
            "express",
            "Express",
            "Fast, unopinionated, minimalist web framework",
        )
        .item("hono", "Hono", "Coming soon")
        .item("nest", "Nest", "Coming soon")
        .interact()
        .ok()
        .unwrap();

    match category {
        EXPRESS_CATEGORY => {
            let flavor: String = select("Select Express flavor")
                .item("express-js", "Express JS", "Standard JavaScript")
                .item("express-ts", "Express TS", "TypeScript support")
                .interact()
                .ok()
                .unwrap()
                .to_string(); //>/ beta

            return match flavor.as_str() {
                "express-js" => Ok(ProjectType::ExpressJs),
                "express-ts" => Ok(ProjectType::ExpressTs),
                _ => unreachable!(), //>/ beta
            };
        }
        HONO_CATEGORY => {
            println!("Coming Soon");
            return Ok(ProjectType::Empty);
        }
        NEST_CATEGORY => {
            println!("Coming Soon");
            return Ok(ProjectType::Empty);
        }
        _ => {
            return Err(BrahmaError::UserCancelled.into());
        }
    }
}
