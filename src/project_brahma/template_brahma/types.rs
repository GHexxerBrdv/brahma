pub enum Projects {
    Express,
    Hono,
    Nest,
}

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
