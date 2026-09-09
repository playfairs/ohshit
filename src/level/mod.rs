use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    OhShit,
}

impl Level {
    pub const WARNING: Self = Self::Warn;

    pub fn is_at_least(self, other: Self) -> bool {
        self >= other
    }

    pub fn color_code(self) -> &'static str {
        match self {
            Self::Trace => "90",
            Self::Debug => "36",
            Self::Info => "32",
            Self::Warn => "33",
            Self::Error => "31",
            Self::OhShit => "35",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trace => write!(f, "TRACE"),
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warn => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
            Self::OhShit => write!(f, "OH SHIT"),
        }
    }
}
