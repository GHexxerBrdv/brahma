pub enum ProjectType {
    Empty,
    ExpressJs,
    ExpressTs,
}

impl ProjectType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Empty => "empty",
            Self::ExpressJs => "express-js",
            Self::ExpressTs => "express-ts",
        }
    }
}
