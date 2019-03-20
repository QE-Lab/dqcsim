//! A generic log proxy implementation.
//!
//! This module defines the [`LogProxy`] struct and implements the [`Log`]
//! trait for it.
//!
//! # Use
//!
//! # Example
//!
//! [`LogProxy`]: ./struct.LogProxy.html
//! [`Log`]: ../trait.Log.html

use crate::log::{Log, Loglevel, LoglevelFilter, Record, Sender};

/// A [`LogProxy`] is a logger implementation (`Log`) which sends log records
/// using its Sender side of a Channel.
///
/// [`LogProxy`]: ./struct.LogProxy.html
#[derive(Debug)]
pub struct LogProxy<T: Sender> {
    name: String,
    level: LoglevelFilter,
    sender: T,
}

impl<T: Sender<Item = Record>> LogProxy<T> {
    fn new(name: impl Into<String>, level: LoglevelFilter, sender: T) -> LogProxy<T> {
        LogProxy {
            name: name.into(),
            level,
            sender,
        }
    }

    /// Return a new boxed LogProxy for the provided sender and level.
    pub fn boxed(name: impl Into<String>, level: LoglevelFilter, sender: T) -> Box<LogProxy<T>> {
        Box::new(LogProxy::new(name, level, sender))
    }
}

impl<T: Sender<Item = Record>> Log for LogProxy<T> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn enabled(&self, level: Loglevel) -> bool {
        LoglevelFilter::from(level) <= self.level
    }
    fn log(&self, record: Record) {
        self.sender
            .send(record)
            .expect("LogProxy failed to send record");
    }
}

#[cfg(test)]
mod tests {
    use crate::log::{proxy::LogProxy, thread::LogThread, Log, Loglevel, LoglevelFilter};

    #[test]
    fn proxy_level() {
        let log_thread = LogThread::spawn(
            "main_thread",
            LoglevelFilter::Off,
            LoglevelFilter::Trace,
            None,
        )
        .unwrap();

        let log_endpoint = log_thread.get_sender().unwrap();

        let log_proxy = LogProxy::boxed("fatal", LoglevelFilter::Fatal, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(!log_proxy.enabled(Loglevel::Error));

        let log_proxy = LogProxy::boxed("error", LoglevelFilter::Error, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(log_proxy.enabled(Loglevel::Error));
        assert!(!log_proxy.enabled(Loglevel::Warn));

        let log_proxy = LogProxy::boxed("warn", LoglevelFilter::Warn, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(log_proxy.enabled(Loglevel::Error));
        assert!(log_proxy.enabled(Loglevel::Warn));
        assert!(!log_proxy.enabled(Loglevel::Note));
        assert!(!log_proxy.enabled(Loglevel::Info));
        assert!(!log_proxy.enabled(Loglevel::Debug));
        assert!(!log_proxy.enabled(Loglevel::Trace));

        let log_proxy = LogProxy::boxed("note", LoglevelFilter::Note, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(log_proxy.enabled(Loglevel::Error));
        assert!(log_proxy.enabled(Loglevel::Warn));
        assert!(log_proxy.enabled(Loglevel::Note));
        assert!(!log_proxy.enabled(Loglevel::Info));
        assert!(!log_proxy.enabled(Loglevel::Debug));
        assert!(!log_proxy.enabled(Loglevel::Trace));

        let log_proxy = LogProxy::boxed("info", LoglevelFilter::Info, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(log_proxy.enabled(Loglevel::Error));
        assert!(log_proxy.enabled(Loglevel::Warn));
        assert!(log_proxy.enabled(Loglevel::Note));
        assert!(log_proxy.enabled(Loglevel::Info));
        assert!(!log_proxy.enabled(Loglevel::Debug));
        assert!(!log_proxy.enabled(Loglevel::Trace));

        let log_proxy = LogProxy::boxed("debug", LoglevelFilter::Debug, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(log_proxy.enabled(Loglevel::Error));
        assert!(log_proxy.enabled(Loglevel::Warn));
        assert!(log_proxy.enabled(Loglevel::Note));
        assert!(log_proxy.enabled(Loglevel::Info));
        assert!(log_proxy.enabled(Loglevel::Debug));
        assert!(!log_proxy.enabled(Loglevel::Trace));

        let log_proxy = LogProxy::boxed("trace", LoglevelFilter::Trace, log_endpoint.clone());
        assert!(log_proxy.enabled(Loglevel::Fatal));
        assert!(log_proxy.enabled(Loglevel::Error));
        assert!(log_proxy.enabled(Loglevel::Warn));
        assert!(log_proxy.enabled(Loglevel::Note));
        assert!(log_proxy.enabled(Loglevel::Info));
        assert!(log_proxy.enabled(Loglevel::Debug));
        assert!(log_proxy.enabled(Loglevel::Trace));

        let log_proxy = LogProxy::boxed("off", LoglevelFilter::Off, log_endpoint.clone());
        assert!(!log_proxy.enabled(Loglevel::Fatal));
        assert!(!log_proxy.enabled(Loglevel::Error));
        assert!(!log_proxy.enabled(Loglevel::Warn));
        assert!(!log_proxy.enabled(Loglevel::Note));
        assert!(!log_proxy.enabled(Loglevel::Info));
        assert!(!log_proxy.enabled(Loglevel::Debug));
        assert!(!log_proxy.enabled(Loglevel::Trace));
    }
}
