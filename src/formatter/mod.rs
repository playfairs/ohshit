pub mod terminal;

pub use self::terminal::TerminalFormatter;

use std::fmt::Debug;

use crate::diagnostic::Diagnostic;
use crate::record::Record;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatterConfig {
    pub colors: bool,
    pub show_location: bool,
    pub show_timestamp: bool,
    pub show_target: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            colors: false,
            show_location: true,
            show_timestamp: true,
            show_target: true,
        }
    }
}

pub trait Formatter: Send + Sync + Debug {
    fn format_record(&self, record: &Record, config: &FormatterConfig) -> String;
    fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String;
}
