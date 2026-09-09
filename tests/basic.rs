use std::sync::Arc;

use ohshit::{Diagnostic, Filter, Level, Logger, LoggerConfig, MemorySink, Record};

#[test]
fn diagnostic_tracks_real_failure_details() {
    let diagnostic = Diagnostic::new("Could not bind to 127.0.0.1:8000")
        .with_cause("Address already in use")
        .with_reason("Another process is already listening on port 8000.")
        .with_action("lsof -i :8000");

    assert_eq!(diagnostic.headline(), "Could not bind to 127.0.0.1:8000");
    assert_eq!(diagnostic.cause(), Some("Address already in use"));
    assert_eq!(diagnostic.reason(), Some("Another process is already listening on port 8000."));
    assert_eq!(diagnostic.action(), Some("lsof -i :8000"));
}

#[test]
fn logger_records_level_and_filtering() {
    let sink = Arc::new(MemorySink::new());
    let logger = Logger::builder()
        .with_config(LoggerConfig::new())
        .filter(Filter::new(Level::Info))
        .build()
        .with_sink(sink.clone());

    logger.info("server started");
    logger.warn("configuration file missing");
    logger.error("database connection failed");

    let records = sink.records();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].level(), Level::Info);
    assert_eq!(records[0].message(), "server started");
    assert_eq!(records[2].level(), Level::Error);
}

#[test]
fn record_supports_context_and_location() {
    let record = Record::new(Level::Warn, "listener failed")
        .with_context("port", 8000)
        .with_location(ohshit::Location::new("examples/basic.rs", 42));

    assert_eq!(record.level(), Level::Warn);
    assert_eq!(record.message(), "listener failed");
    assert_eq!(record.location().map(|loc| loc.file()), Some("examples/basic.rs"));
    assert_eq!(record.location().map(|loc| loc.line()), Some(42));
    assert_eq!(record.context().entries().len(), 1);
}
