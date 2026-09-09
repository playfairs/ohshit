use crate::diagnostic::Diagnostic;
use crate::diagnostic::Location;
use crate::level::Level;
use crate::record::context::Context;
use crate::record::metadata::RecordMetadata;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    level: Level,
    target: String,
    message: String,
    metadata: RecordMetadata,
    context: Context,
}

impl Record {
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        Self {
            level,
            target: "ohshit".to_string(),
            message: message.into(),
            metadata: RecordMetadata::new(),
            context: Context::new(),
        }
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = target.into();
        self
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn location(&self) -> Option<&Location> {
        self.metadata.location()
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.metadata = self.metadata.with_location(location);
        self
    }

    pub fn with_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.metadata = self.metadata.with_diagnostic(diagnostic);
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl std::fmt::Display) -> Self {
        self.context.insert(key, value);
        self
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn metadata(&self) -> &RecordMetadata {
        &self.metadata
    }
}
