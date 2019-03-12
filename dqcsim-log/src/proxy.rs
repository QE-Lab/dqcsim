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

use crate::{Level, LevelFilter, Log, Record, Sender};

/// A [`LogProxy`] is a logger implementation (`Log`) which sends log records
/// using its Sender side of a Channel.
///
/// [`LogProxy`]: ./struct.LogProxy.html
pub struct LogProxy<T: Sender> {
    level: LevelFilter,
    sender: T,
}

impl<T: Sender<Item = Record>> LogProxy<T> {
    fn new(sender: T, level_filter: Option<LevelFilter>) -> LogProxy<T> {
        LogProxy {
            sender,
            level: level_filter.unwrap_or(LevelFilter::Info),
        }
    }

    /// Return a new boxed LogProxy for the provided sender and level.
    pub fn boxed(sender: T, level_filter: Option<LevelFilter>) -> Box<LogProxy<T>> {
        Box::new(LogProxy::new(sender, level_filter))
    }
}

impl<T: Sender<Item = Record>> Log for LogProxy<T> {
    fn enabled(&self, level: Level) -> bool {
        LevelFilter::from(level) <= self.level
    }

    fn log(&self, record: Record) {
        if self.enabled(record.level()) {
            self.sender
                .send(record)
                .expect("LogProxy failed to send record");
        }
    }
}
