use std::time::{SystemTime, UNIX_EPOCH};

use crate::diagnostic::Diagnostic;
use crate::diagnostic::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordMetadata {
    timestamp: SystemTime,
    location: Option<Location>,
    diagnostic: Option<Diagnostic>,
}

impl RecordMetadata {
    pub fn new() -> Self {
        Self {
            timestamp: SystemTime::now(),
            location: None,
            diagnostic: None,
        }
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }

    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn diagnostic(&self) -> Option<&Diagnostic> {
        self.diagnostic.as_ref()
    }

    pub fn timestamp_seconds(&self) -> u64 {
        self.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl Default for RecordMetadata {
    fn default() -> Self {
        Self::new()
    }
}
