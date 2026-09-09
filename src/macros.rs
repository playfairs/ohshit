#[macro_export]
macro_rules! trace {
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Trace, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Trace, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Debug, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Debug, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Info, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Info, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Warn, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Warn, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit!($crate::Level::Error, $fmt $(, $arg)*);
    }};
    ($($key:ident = $value:expr),+ ; $fmt:literal $(, $arg:expr)*) => {{
        $crate::__emit_with_fields!($crate::Level::Error, [$($key = $value),+], $fmt $(, $arg)*);
    }};
}

#[macro_export]
macro_rules! ohshit {
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
        let diagnostic = $crate::Diagnostic::new(format!($fmt $(, $arg)*));
        $crate::__log_diagnostic(diagnostic);
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
pub fn __log_record(record: crate::Record) {
    let logger = crate::global::logger();
    logger.log(record);
}

#[doc(hidden)]
pub fn __log_diagnostic(diagnostic: crate::Diagnostic) {
    let logger = crate::global::logger();
    logger.ohshit(diagnostic);
}
