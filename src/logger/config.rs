use crate::filter::Filter;
use crate::level::Level;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerConfig {
    pub level: Level,
    pub filter: Filter,
    pub colors: bool,
    pub show_location: bool,
    pub show_timestamp: bool,
}

impl LoggerConfig {
    pub fn new() -> Self {
        Self {
            level: Level::Info,
            filter: Filter::default(),
            colors: false,
            show_location: true,
            show_timestamp: true,
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self::new()
    }
}
