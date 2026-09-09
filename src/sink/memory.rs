use std::sync::{Arc, Mutex};

use crate::record::Record;
use crate::sink::Sink;

#[derive(Clone, Debug, Default)]
pub struct MemorySink {
    records: Arc<Mutex<Vec<Record>>>,
}

impl MemorySink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn records(&self) -> Vec<Record> {
        self.records.lock().unwrap().clone()
    }
}

impl Sink for MemorySink {
    fn emit(&self, record: Record) {
        self.records.lock().unwrap().push(record);
    }
}
