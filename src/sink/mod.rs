pub mod console;
pub mod memory;

pub use self::console::ConsoleSink;
pub use self::memory::MemorySink;

use std::fmt::Debug;

use crate::record::Record;

pub trait Sink: Send + Sync + Debug {
    fn emit(&self, record: Record);
}
