use crate::filter::Filter;
use crate::level::Level;
use crate::logger::config::LoggerConfig;
use crate::logger::logger::Logger;

#[derive(Clone, Debug, Default)]
pub struct LoggerBuilder {
    config: LoggerConfig,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: LoggerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn level(mut self, level: Level) -> Self {
        self.config.level = level;
        self.config.filter = Filter::new(level);
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.config.filter = filter;
        self
    }

    pub fn build(self) -> Logger {
        Logger::new(self.config)
    }
}
