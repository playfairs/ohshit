use crate::diagnostic::Diagnostic;
use crate::level::Level;
use crate::record::Record;

#[derive(Clone, Debug, Default)]
pub struct TerminalFormatter;

impl TerminalFormatter {
    pub fn format_record(&self, record: &Record) -> String {
        let mut output = Vec::new();
        output.push(format!("{} {}", record.level(), record.message()));

        if let Some(location) = record.metadata().location() {
            output.push(format!("Location: {}:{}", location.file(), location.line()));
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
