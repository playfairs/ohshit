use std::sync::{Arc, Mutex};

use crate::formatter::TerminalFormatter;
use crate::record::Record;
use crate::sink::Sink;

#[derive(Debug, Default)]
pub struct ConsoleSink {
    formatter: TerminalFormatter,
    output: Arc<Mutex<Vec<String>>>,
}

impl Clone for ConsoleSink {
    fn clone(&self) -> Self {
        Self {
            formatter: self.formatter.clone(),
            output: Arc::clone(&self.output),
        }
    }
}

impl ConsoleSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> Vec<String> {
        self.output.lock().unwrap().clone()
    }
}

impl Sink for ConsoleSink {
    fn emit(&self, record: Record) {
        let rendered = self.formatter.format_record(&record);
        println!("{rendered}");
        self.output.lock().unwrap().push(rendered);
    }
}
