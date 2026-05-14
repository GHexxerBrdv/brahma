#[derive(Debug, PartialEq)]
pub enum Projects {
    Express,
    Hono,
    Nest,
}
#[derive(Debug, PartialEq)]
pub enum ProjectFlavors {
    None,
    ExpressJs,
    ExpressTs,
}

impl ProjectFlavors {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::ExpressJs => "express-js",
            Self::ExpressTs => "express-ts",
        }
    }
}
