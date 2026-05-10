pub enum Projects {
    Empty,
    Express,
    Hono,
    Nest,
}

pub enum ProjectFlavors {
    None,
    // Empty,
    ExpressJs,
    ExpressTs,
}

impl ProjectFlavors {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            // Self::Empty => "empty",
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
