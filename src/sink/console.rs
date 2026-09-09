use std::sync::{Arc, Mutex};

use crate::formatter::{Formatter, FormatterConfig, TerminalFormatter};
use crate::record::Record;
use crate::sink::Sink;

#[derive(Debug)]
pub struct ConsoleSink {
    formatter: Arc<dyn Formatter>,
    config: FormatterConfig,
    output: Arc<Mutex<Vec<String>>>,
}

impl Default for ConsoleSink {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ConsoleSink {
    fn clone(&self) -> Self {
        Self {
            formatter: Arc::clone(&self.formatter),
            config: self.config.clone(),
            output: Arc::clone(&self.output),
        }
    }
}

impl ConsoleSink {
    pub fn new() -> Self {
        Self {
            formatter: Arc::new(TerminalFormatter::default()),
            config: FormatterConfig::default(),
            output: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_formatter(formatter: Arc<dyn Formatter>) -> Self {
        Self {
            formatter,
            ..Self::new()
        }
    }

    pub fn with_config(mut self, config: FormatterConfig) -> Self {
        self.config = config;
        self
    }

    pub fn entries(&self) -> Vec<String> {
        self.output.lock().unwrap().clone()
    }
}

impl Sink for ConsoleSink {
    fn emit(&self, record: Record) {
        let rendered = self.formatter.format_record(&record, &self.config);
        let mut output = self.output.lock().unwrap();
        println!("{rendered}");
        output.push(rendered);
    }
}
