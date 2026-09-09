#![doc = "OhSh*t is an experimental Rust logging and diagnostics library. See README.rst for usage and development information."]

mod diagnostic;
mod error;
mod filter;
mod formatter;
mod global;
mod level;
mod logger;
mod macros;
mod record;
mod sink;

pub use diagnostic::{Action, Cause, Diagnostic, Location};
pub use error::{OhShitError, Result};
pub use filter::Filter;
pub use formatter::{Formatter, FormatterConfig, TerminalFormatter};
pub use level::Level;
pub use logger::{Logger, LoggerBuilder, LoggerConfig};
pub use macros::{__log_diagnostic, __log_diagnostic_target, __log_record};
pub use record::{Context, Record, RecordMetadata};
pub use sink::{ConsoleSink, MemorySink, Sink};

pub type LogLevel = Level;
pub type SourceLocation = Location;
pub type LogRecord = Record;

pub fn init() -> Logger {
    global::install_default()
}

pub fn default_logger() -> Logger {
    global::logger()
}

pub fn install_logger(logger: Logger) {
    global::install(logger);
}
