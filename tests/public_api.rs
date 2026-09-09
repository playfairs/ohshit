use std::sync::Arc;
use std::thread;

use ohshit::install_logger;
use ohshit::{
    Diagnostic, Filter, FormatterConfig, Level, Logger, LoggerConfig, MemorySink, Record,
    TerminalFormatter,
};

#[test]
fn public_macros_create_structured_records() {
    let sink = Arc::new(MemorySink::new());
    let config = LoggerConfig {
        level: Level::Trace,
        filter: Filter::new(Level::Trace),
        ..LoggerConfig::default()
    };
    install_logger(
        Logger::builder()
            .with_config(config)
            .build()
            .with_sink(sink.clone()),
    );

    ohshit::trace!(target: "test", field = "trace"; "trace message");
    ohshit::debug!(target: "test", field = "debug"; "debug message");
    ohshit::info!(target: "test", field = "info"; "info message");
    ohshit::warn!(target: "test", field = "warn"; "warn message");
    ohshit::error!(target: "test", field = "error"; "error message");
    let diagnostic = Diagnostic::new("serious failure")
        .with_cause("real cause")
        .with_reason("verified reason")
        .with_action("take corrective action")
        .with_location(ohshit::Location::new("tests/public_api.rs", 24));
    ohshit::ohshit!(target: "test", diagnostic);

    let records = sink.records();
    assert_eq!(records.len(), 6);
    assert_eq!(records[0].level(), Level::Trace);
    assert_eq!(records[0].target(), "test");
    assert_eq!(records[0].context().entries()[0].0, "field");
    assert!(records[0].metadata().timestamp_seconds() > 0);
    assert!(records[0].location().is_some());
    assert_eq!(records[5].level(), Level::OhShit);
    assert_eq!(
        records[5].metadata().diagnostic().unwrap().cause(),
        Some("real cause")
    );
    assert_eq!(records[5].location().unwrap().file(), "tests/public_api.rs");
}

#[test]
fn formatter_honors_presentation_configuration() {
    let record = Record::new(Level::Info, "message").with_target("test");
    let formatter = TerminalFormatter::default();
    let output = formatter.format_record_with_config(
        &record,
        &FormatterConfig {
            show_timestamp: false,
            show_target: false,
            show_location: false,
            colors: false,
        },
    );
    assert_eq!(output, "INFO message");
}

#[test]
fn logger_dispatch_is_thread_safe() {
    let sink = Arc::new(MemorySink::new());
    let logger = Logger::builder()
        .level(Level::Info)
        .build()
        .with_sink(sink.clone());
    let workers = (0..4)
        .map(|worker| {
            let logger = logger.clone();
            thread::spawn(move || {
                for sequence in 0..25 {
                    logger.log(
                        Record::new(Level::Info, "concurrent")
                            .with_target("thread")
                            .with_context("worker", worker)
                            .with_context("sequence", sequence),
                    );
                }
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker.join().expect("logging worker should finish");
    }
    assert_eq!(sink.records().len(), 100);
}
