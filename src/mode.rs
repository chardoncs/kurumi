/// Vim mode
#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    //Insert,
    //Command,
    //Visual,
    //LineVisual,
    //BlockVisual,
    //Replace,
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Normal => "normal",
            //Self::Insert => "insert",
            //Self::Command => "command",
            //Self::Visual => "visual",
            //Self::LineVisual => "line visual",
            //Self::BlockVisual => "block visual",
            //Self::Replace => "replace",
        }
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
