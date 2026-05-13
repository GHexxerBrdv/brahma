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
    ExpressJsNoGit,
    ExpressTs,
    ExpressTsNoGit,
}

impl ProjectFlavors {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::ExpressJs => "express-js",
            Self::ExpressJsNoGit => "express-js-no-git",
            Self::ExpressTs => "express-ts",
            Self::ExpressTsNoGit => "express-ts-no-git",
        }
    }
}

// impl Projects {
//     pub fn as_str(&self) -> &str {
//         match self {
//             Self::Empty => "empty",
//             Self::Express => "express",
//             Self::Hono => "hono",
//             Self::Nest => "nest",
//         }
//     }
// }
