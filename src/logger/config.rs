use crate::filter::Filter;
use crate::level::Level;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerConfig {
    pub level: Level,
    pub filter: Filter,
    pub colors: bool,
    pub show_location: bool,
    pub show_timestamp: bool,
    pub show_target: bool,
}

impl LoggerConfig {
    pub fn new() -> Self {
        Self {
            level: Level::Info,
            filter: Filter::default(),
            colors: false,
            show_location: true,
            show_timestamp: true,
            show_target: true,
        }
    }

    pub fn formatter_config(&self) -> crate::FormatterConfig {
        crate::FormatterConfig {
            colors: self.colors,
            show_location: self.show_location,
            show_timestamp: self.show_timestamp,
            show_target: self.show_target,
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self::new()
    }
}
