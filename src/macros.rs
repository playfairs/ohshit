#[macro_export]
macro_rules! trace {
    (target: $target:literal, $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target!($crate::Level::Trace, $target, $fmt $(, $arg)*);
    }};
    (target: $target:literal, $($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target_with_fields!($crate::Level::Trace, $target, [$($key = $value),+], $fmt $(, $arg)*);
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Trace, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Trace, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! debug {
    (target: $target:literal, $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target!($crate::Level::Debug, $target, $fmt $(, $arg)*);
    }};
    (target: $target:literal, $($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target_with_fields!($crate::Level::Debug, $target, [$($key = $value),+], $fmt $(, $arg)*);
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Debug, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Debug, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! info {
    (target: $target:literal, $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target!($crate::Level::Info, $target, $fmt $(, $arg)*);
    }};
    (target: $target:literal, $($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target_with_fields!($crate::Level::Info, $target, [$($key = $value),+], $fmt $(, $arg)*);
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Info, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Info, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! warn {
    (target: $target:literal, $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target!($crate::Level::Warn, $target, $fmt $(, $arg)*);
    }};
    (target: $target:literal, $($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target_with_fields!($crate::Level::Warn, $target, [$($key = $value),+], $fmt $(, $arg)*);
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Warn, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Warn, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! error {
    (target: $target:literal, $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target!($crate::Level::Error, $target, $fmt $(, $arg)*);
    }};
    (target: $target:literal, $($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_target_with_fields!($crate::Level::Error, $target, [$($key = $value),+], $fmt $(, $arg)*);
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Error, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Error, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! ohshit {
    (target: $target:literal, $fmt:literal $(, $arg:expr)*) => {{
        $crate::__ohshit_emit_target!($target, $fmt $(, $arg)*);
    }};
    (target: $target:literal, $($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__ohshit_emit_target_with_fields!($target, [$($key = $value),+], $fmt $(, $arg)*);
    }};
    (target: $target:literal, $diagnostic:expr) => {{
        $crate::__log_diagnostic_target($target, $diagnostic);
    }};
    ($diagnostic:expr) => {{
        $crate::__log_diagnostic($diagnostic);
    }};
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__ohshit_emit!($fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__ohshit_emit_with_fields!([$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit {
    ($level:expr, $fmt:literal $(, $arg:expr)*) => {{
        let mut record = $crate::Record::new($level, format!($fmt $(, $arg)*));
        record = record.with_location($crate::Location::new(file!(), line!()));
        $crate::__log_record(record);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit_target {
    ($level:expr, $target:expr, $fmt:literal $(, $arg:expr)*) => {{
        let record = $crate::Record::new($level, format!($fmt $(, $arg)*))
            .with_target($target)
            .with_location($crate::Location::new(file!(), line!()));
        $crate::__log_record(record);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit_target_with_fields {
    ($level:expr, $target:expr, [$($key:ident = $value:expr),+], $fmt:literal $(, $arg:expr)*) => {{
        let mut record = $crate::Record::new($level, format!($fmt $(, $arg)*))
            .with_target($target)
            .with_location($crate::Location::new(file!(), line!()));
        $(record = record.with_context(stringify!($key), $value);)+
        $crate::__log_record(record);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit_with_fields {
    ($level:expr, [$($key:ident = $value:expr),+], $fmt:literal $(, $arg:expr)*) => {{
        let mut record = $crate::Record::new($level, format!($fmt $(, $arg)*));
        record = record.with_location($crate::Location::new(file!(), line!()));
        $(
            record = record.with_context(stringify!($key), format!("{}", $value));
        )+
        $crate::__log_record(record);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ohshit_emit {
    ($fmt:literal $(, $arg:expr)*) => {{
        let diagnostic = $crate::Diagnostic::new(format!($fmt $(, $arg)*))
            .with_location($crate::Location::new(file!(), line!()));
        $crate::__log_diagnostic(diagnostic);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ohshit_emit_target {
    ($target:expr, $fmt:literal $(, $arg:expr)*) => {{
        let diagnostic = $crate::Diagnostic::new(format!($fmt $(, $arg)*))
            .with_location($crate::Location::new(file!(), line!()));
        $crate::__log_diagnostic_target($target, diagnostic);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ohshit_emit_with_fields {
    ([$($key:ident = $value:expr),+], $fmt:literal $(, $arg:expr)*) => {{
        let mut diagnostic = $crate::Diagnostic::new(format!($fmt $(, $arg)*));
        diagnostic = diagnostic.with_location($crate::Location::new(file!(), line!()));
        $(
            diagnostic = diagnostic.with_context(stringify!($key), format!("{}", $value));
        )+
        $crate::__log_diagnostic(diagnostic);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ohshit_emit_target_with_fields {
    ($target:expr, [$($key:ident = $value:expr),+], $fmt:literal $(, $arg:expr)*) => {{
        let mut diagnostic = $crate::Diagnostic::new(format!($fmt $(, $arg)*))
            .with_location($crate::Location::new(file!(), line!()));
        $(diagnostic = diagnostic.with_context(stringify!($key), $value);)+
        $crate::__log_diagnostic_target($target, diagnostic);
    }};
}

#[doc(hidden)]
pub fn __log_record(record: crate::Record) {
    let logger = crate::global::logger();
    logger.log(record);
}

#[doc(hidden)]
pub fn __log_diagnostic(diagnostic: crate::Diagnostic) {
    let logger = crate::global::logger();
    logger.ohshit(diagnostic);
}

#[doc(hidden)]
pub fn __log_diagnostic_target(target: impl Into<String>, diagnostic: crate::Diagnostic) {
    let logger = crate::global::logger();
    logger.ohshit_target(target, diagnostic);
}
