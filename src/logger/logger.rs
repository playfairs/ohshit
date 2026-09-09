use std::sync::Arc;

use crate::diagnostic::Diagnostic;
use crate::diagnostic::Location;
use crate::filter::Filter;
use crate::level::Level;
use crate::logger::config::LoggerConfig;
use crate::record::Record;
use crate::sink::Sink;

#[derive(Clone, Debug)]
pub struct Logger {
    config: LoggerConfig,
    sinks: Vec<Arc<dyn Sink>>,
}

impl Logger {
    pub fn new(config: LoggerConfig) -> Self {
        Self {
            config,
            sinks: Vec::new(),
        }
    }

    pub fn builder() -> crate::logger::builder::LoggerBuilder {
        crate::logger::builder::LoggerBuilder::new()
    }

    pub fn with_config(mut self, config: LoggerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_sink(mut self, sink: Arc<dyn Sink>) -> Self {
        self.sinks.push(sink);
        self
    }

    pub fn config(&self) -> &LoggerConfig {
        &self.config
    }

    pub fn filter(&self) -> &Filter {
        &self.config.filter
    }

    pub fn record(
        &self,
        level: Level,
        message: impl Into<String>,
        location: Option<Location>,
    ) -> Record {
        let mut record = Record::new(level, message);
        if let Some(location) = location {
            record = record.with_location(location);
        }
        record
    }

    pub fn record_with_context(
        &self,
        level: Level,
        message: impl Into<String>,
        location: Option<Location>,
        context: Vec<(String, String)>,
    ) -> Record {
        let mut record = self.record(level, message, location);
        for (key, value) in context {
            record = record.with_context(key, value);
        }
        record
    }

    pub fn render_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        diagnostic.render()
    }

    pub fn log(&self, record: Record) {
        if !self.config.filter.allows(record.level()) {
            return;
        }

        for sink in &self.sinks {
            sink.emit(record.clone());
        }
    }

    pub fn info(&self, message: impl Into<String>) {
        self.log(Record::new(Level::Info, message));
    }

    pub fn warn(&self, message: impl Into<String>) {
        self.log(Record::new(Level::Warn, message));
    }

    pub fn error(&self, message: impl Into<String>) {
        self.log(Record::new(Level::Error, message));
    }

    pub fn ohshit(&self, diagnostic: Diagnostic) {
        self.ohshit_target("ohshit", diagnostic);
    }

    pub fn ohshit_target(&self, target: impl Into<String>, diagnostic: Diagnostic) {
        let mut record = Record::new(Level::OhShit, diagnostic.headline().to_string())
            .with_target(target)
            .with_diagnostic(diagnostic.clone());
        if let Some(location) = diagnostic.location().cloned() {
            record = record.with_location(location);
        }
        self.log(record);
    }
}
