use std::sync::{Arc, OnceLock, RwLock};

use crate::logger::Logger;
use crate::sink::ConsoleSink;

static LOGGER: OnceLock<RwLock<Logger>> = OnceLock::new();

pub fn install_default() -> Logger {
    let logger = Logger::builder()
        .level(crate::Level::Info)
        .build()
        .with_sink(Arc::new(ConsoleSink::new()));
    install(logger.clone());
    logger
}

pub fn install(logger: Logger) {
    let slot = LOGGER.get_or_init(|| RwLock::new(Logger::builder().level(crate::Level::Info).build()));
    let mut guard = slot.write().unwrap();
    *guard = logger;
}

pub fn logger() -> Logger {
    let slot = LOGGER.get_or_init(|| RwLock::new(Logger::builder().level(crate::Level::Info).build()));
    let guard = slot.read().unwrap();
    guard.clone()
}
