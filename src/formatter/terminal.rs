use crate::diagnostic::Diagnostic;
use crate::formatter::{Formatter, FormatterConfig};
use crate::level::Level;
use crate::record::Record;

#[derive(Clone, Debug, Default)]
pub struct TerminalFormatter;

impl TerminalFormatter {
    pub fn format_record(&self, record: &Record) -> String {
        self.format_record_with_config(record, &FormatterConfig::default())
    }

    pub fn format_record_with_config(&self, record: &Record, config: &FormatterConfig) -> String {
        let mut output = Vec::new();
        let level = if config.colors {
            format!(
                "\x1b[{}m{}\x1b[0m",
                record.level().color_code(),
                record.level()
            )
        } else {
            record.level().to_string()
        };
        let timestamp = if config.show_timestamp {
            format!("[{}] ", record.metadata().timestamp_seconds())
        } else {
            String::new()
        };
        let target = if config.show_target {
            format!(" target={} ", record.target())
        } else {
            " ".to_string()
        };
        output.push(format!("{timestamp}{level}{target}{}", record.message()));

        if config.show_location {
            if let Some(location) = record.metadata().location() {
                output.push(format!("Location: {}:{}", location.file(), location.line()));
            }
        }

        if let Some(diagnostic) = record.metadata().diagnostic() {
            output.push(String::new());
            output.push(diagnostic.render());
        }

        output.join("\n")
    }

    pub fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        diagnostic.render()
    }

    pub fn format_level(&self, level: Level) -> String {
        level.to_string()
    }
}

impl Formatter for TerminalFormatter {
    fn format_record(&self, record: &Record, config: &FormatterConfig) -> String {
        self.format_record_with_config(record, config)
    }

    fn format_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        diagnostic.render()
    }
}
