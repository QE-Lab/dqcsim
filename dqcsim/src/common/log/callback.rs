use crate::common::log::{Log, LogRecord, Loglevel, LoglevelFilter};

/// Log callback function structure.
///
/// Note the lack of derives; they don't play well with `Box<dyn Fn...>`...
/// I wonder why. That's primarily why this struct is defined outside
/// `SimulatorConfiguration`.
pub struct LogCallback {
    /// The callback function to call.
    ///
    /// The sole argument is the log message record.
    pub callback: Box<dyn Fn(&LogRecord) + Send>,

    /// Verbosity level for calling the log callback function.
    pub filter: LoglevelFilter,
}

impl LogCallback {
    /// Constructs a new LogCallback.
    pub fn new(callback: Box<dyn Fn(&LogRecord) + Send>, filter: LoglevelFilter) -> LogCallback {
        LogCallback { callback, filter }
    }
}

impl Log for LogCallback {
    fn name(&self) -> &str {
        "?"
    }
    fn enabled(&self, level: Loglevel) -> bool {
        LoglevelFilter::from(level) <= self.filter
    }
    fn log(&self, record: &LogRecord) {
        (self.callback)(record);
    }
}

impl std::fmt::Debug for LogCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "LogCallback {{ callback: <...>, filter: {:?} }}",
            self.filter
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_callback() {
        let record = LogRecord::new(
            "logger",
            "message",
            Loglevel::Trace,
            "path",
            "file",
            1234u32,
            1u32,
            1u64,
        );
        let callback = LogCallback::new(
            Box::new(|record| {
                assert_eq!(record.logger(), "logger");
                assert_eq!(record.line(), Some(1234u32));
            }),
            LoglevelFilter::Debug,
        );
        (callback.callback)(&record);
        assert_eq!(callback.name(), "?");
        assert!(callback.enabled(Loglevel::Debug));
        assert!(!callback.enabled(Loglevel::Trace));
        callback.log(&record);
        assert_eq!(
            format!("{:?}", callback),
            "LogCallback { callback: <...>, filter: Debug }"
        );
    }
}
