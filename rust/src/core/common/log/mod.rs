//! A log thread and thread-local log proxy combination.
//!
//! This module provides logging functionality to run a dedicated log thread in
//! combination with one or more thread-local log proxy instances. The log
//! thread provides the endpoint used by the log proxy instances to send their
//! log records. Log proxy instances run in different threads or child
//! processes.
//!
//! # Usage
//!
//! Start by spawning a [`LogThread`] from the main thread. Next, initialize a
//! [`LogProxy`] instance per thread or child process. A log [`LogRecord`] can be
//! generated using the provided [`macros`]. The thread-local [`LogProxy`]
//! forwards the records to the [`LogThread`] for logging.
//!
//! ## LogThread
//!
//! The [`LogThread`] is the sink for all log [`Records`]. It can output log
//! records to Standard Error output and invoke a [`LogCallback`] function.
//! Both these options can be enabled by setting the corresponding
//! [`LogLevelFilter`] above [`LogLevelFilter::Off`]. Incoming log [`Records`]
//! are forwarded to Standard Error output or to the [`LogCallback`] function
//! if their [`Loglevel`] is equal or below the configured [`LogLevelFilter`].
//!
//! ## LogCallback
//!
//! A [`LogThread`] can invoke a [`LogCallback`] function for incoming records.
//! This is enabled by passing a [`LogCallback`] (with a [`LoglevelFilter`]
//! bigger than [`LogLevelFilter::Off`]) to the [`callback`] argument of the
//! [`spawn`] method of [`LogThread`].
//!
//! ## LogProxy
//!
//! A [`LogProxy`] forwards log [`Records`] to a [`LogThread`]. It logs records
//! with it's logger name if the generated log [`LogRecord`] [`Loglevel`] is
//! smaller or equal than the configured [`LoglevelFilter`] of the
//! [`LogProxy`].
//!
//! ## TeeFile
//!
//! A [`TeeFile`] forwards log [`Records`] to a file. It logs records with it's
//! logger name if the generated log [`LogRecord`] [`Loglevel`] is smaller or
//! equal than the configured [`LoglevelFilter`] of the [`TeeFile`].
//!
//! # Basic Example
//!
//! ```rust
//! use dqcsim::{
//!     debug,
//!     common::log::{init, proxy::LogProxy, thread::LogThread, LoglevelFilter},
//!     note,
//! };
//!
//! // Spawn the log thread. This starts a thread-local log proxy in the main
//! // thread with a Note level filter and "main_thread" as name. This example
//! // enables Standard Error output at Debug level filter.
//! let log_thread = LogThread::spawn(
//!     "main_thread",
//!     LoglevelFilter::Note,
//!     LoglevelFilter::Debug,
//!     None,
//!     vec![]
//! )
//! .unwrap();
//!
//! // Grab a copy of the log thread sender to use in the log proxy.
//! let log_endpoint = log_thread.get_sender();
//!
//! // Spawn an other thread.
//! std::thread::spawn(move || {
//!     // Construct a log proxy instance which connects to the log thread endpoint.
//!     let log_proxy = LogProxy::boxed("other_thread", LoglevelFilter::Trace, log_endpoint);
//!
//!     // Initialize the thread-local logger to enable forwarding of log records to
//!     // the log thread.
//!     init(vec![log_proxy]);
//!
//!     // Generate a log record
//!     note!("Note from thread via proxy");
//! })
//! .join();
//!
//! // This log records is also forwarded to the log thread by the log proxy running
//! // in the main thread.
//! debug!("Note from main thread via proxy started by log_thread spawn function");
//!
//! ```
//!
//! # Inspired by
//! * [`log`]
//! * sfackler's [comment](https://github.com/rust-lang-nursery/log/issues/57#issuecomment-143383896)
//!
//! [`LogThread`]: ./thread/struct.LogThread.html
//! [`spawn`]: ./thread/struct.LogThread.html#method.spawn
//! [`LogProxy`]: ./proxy/struct.LogProxy.html
//! [`TeeFile`]: ./tee_file/struct.TeeFile.html
//! [`LogRecord`]: ./struct.LogRecord.html
//! [`Records`]: ./struct.LogRecord.html
//! [`Loglevel`]: ./enum.Loglevel.html
//! [`LoglevelFilter`]: ./enum.LoglevelFilter.html
//! [`LogLevelFilter::Off`]: ./enum.LogLevelFilter.html
//! [`LogCallback`]: ../configuration/struct.LogCallback.html
//! [`macros`]: ../index.html#macros
//! [`log`]: https://github.com/rust-lang-nursery/log

// This re-export is required as the trait needs to be in scope in the log
// macro.
#[doc(hidden)]
pub use ref_thread_local as _ref_thread_local;

pub mod callback;
pub mod proxy;
pub mod stdio;
pub mod tee_file;
pub mod thread;

use crate::common::{
    channel::Sender,
    error,
    error::{ErrorKind, ResultExt},
};
use lazy_static::lazy_static;
use named_type::NamedType;
use named_type_derive::*;
use ref_thread_local::ref_thread_local;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt};
use strum_macros::{Display, EnumIter, EnumString};

/// The Log trait.
///
/// # Implementing the Log trait.
///
/// ```rust
/// use dqcsim::{
///     common::log::{Log, Loglevel, LogRecord}
/// };
///
/// struct SimpleLogger {}
///
/// impl Log for SimpleLogger {
///     fn name(&self) -> &str {
///         "simple_logger"
///     }
///
///     fn enabled(&self, level: Loglevel) -> bool {
///         // The SimpleLogger is always enabled.
///         true
///     }
///
///     fn log(&self, record: &LogRecord) {
///         // The SimpleLogger logs to Standard Output.
///         println!("{}", record);
///     }
/// }
/// ```
pub trait Log {
    /// Returns the name of this logger
    fn name(&self) -> &str;
    /// Returns true if the provided loglevel is enabled
    fn enabled(&self, level: Loglevel) -> bool;
    /// Log the incoming record
    fn log(&self, record: &LogRecord);
}

thread_local! {
    /// The thread-local loggers.
    pub static LOGGERS: RefCell<Option<Vec<Box<dyn Log>>>> = RefCell::new(None);
}

lazy_static! {
    // Cache the process id.
    #[doc(hidden)]
    pub static ref PID: u32 = std::process::id();
}

ref_thread_local! {
    // Cache the thread id.
    #[doc(hidden)]
    // Don't ask. (rust-lang/rust #52780)
    pub static managed TID: u64 = unsafe { std::mem::transmute(std::thread::current().id()) };
}

/// Loglevel for log records.
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    EnumIter,
    NamedType,
)]
pub enum Loglevel {
    /// This loglevel is to be used for reporting a fatal error, resulting from
    /// the owner of the logger getting into an illegal state from which it
    /// cannot recover. Such problems are also reported to the API caller via
    /// Result::Err if applicable.
    Fatal = 1,

    /// This loglevel is to be used for reporting or propagating a non-fatal
    /// error caused by the API caller doing something wrong. Such problems are
    /// also reported to the API caller via Result::Err if applicable.
    Error,

    /// This loglevel is to be used for reporting that a called API/function is
    /// telling us we did something wrong (that we weren't expecting), but we
    /// can recover. For instance, for a failed connection attempt to something
    /// that really should not be failing, we can still retry (and eventually
    /// report critical or error if a retry counter overflows). Since we're
    /// still trying to rectify things at this point, such problems are NOT
    /// reported to the API/function caller via Result::Err.
    Warn,

    /// This loglevel is to be used for reporting information specifically
    /// requested by the user/API caller, such as the result of an API function
    /// requested through the command line, or an explicitly captured
    /// stdout/stderr stream.
    Note,

    /// This loglevel is to be used for reporting information NOT specifically
    /// requested by the user/API caller, such as a plugin starting up or
    /// shutting down.
    Info,

    /// This loglevel is to be used for reporting debugging information useful
    /// for debugging the user of the API provided by the logged instance.
    Debug,

    /// This loglevel is to be used for reporting debugging information useful
    /// for debugging the internals of the logged instance. Such messages would
    /// normally only be generated by debug builds, to prevent them from
    /// impacting performance under normal circumstances.
    Trace,
}

impl Into<term::color::Color> for Loglevel {
    fn into(self) -> term::color::Color {
        match self {
            Loglevel::Fatal => term::color::BRIGHT_RED,
            Loglevel::Error => term::color::RED,
            Loglevel::Warn => term::color::YELLOW,
            Loglevel::Note => term::color::WHITE,
            Loglevel::Info => term::color::BLUE,
            Loglevel::Debug => term::color::CYAN,
            Loglevel::Trace => term::color::BRIGHT_BLACK,
        }
    }
}

/// LoglevelFilter for implementors of the Log trait.
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    EnumIter,
    NamedType,
)]
pub enum LoglevelFilter {
    /// A level lower than all log levels.
    #[strum(to_string = "Off", serialize = "off", serialize = "o")]
    Off = 0,
    /// Corresponds to the `Fatal` log level.
    #[strum(to_string = "Fatal", serialize = "fatal", serialize = "f")]
    Fatal,
    /// Corresponds to the `Error` log level.
    #[strum(to_string = "Error", serialize = "error", serialize = "e")]
    Error,
    /// Corresponds to the `Warn` log level.
    #[strum(to_string = "Warn", serialize = "warn", serialize = "w")]
    Warn,
    /// Corresponds to the `Note` log level.
    #[strum(to_string = "Note", serialize = "note", serialize = "n")]
    Note,
    /// Corresponds to the `Info` log level.
    #[strum(to_string = "Info", serialize = "info", serialize = "i")]
    Info,
    /// Corresponds to the `Debug` log level.
    #[strum(to_string = "Debug", serialize = "debug", serialize = "d")]
    Debug,
    /// Corresponds to the `Trace` log level.
    #[strum(to_string = "Trace", serialize = "trace", serialize = "t")]
    Trace,
}

impl Loglevel {
    /// Attempt to convert a LoglevelFilter to a Loglevel.
    ///
    /// Until std::convert::TryFrom is stable. (rust-lang/rust #33417)
    pub fn try_from(levelfilter: LoglevelFilter) -> Result<Loglevel, ()> {
        match levelfilter {
            LoglevelFilter::Fatal => Ok(Loglevel::Fatal),
            LoglevelFilter::Error => Ok(Loglevel::Error),
            LoglevelFilter::Warn => Ok(Loglevel::Warn),
            LoglevelFilter::Note => Ok(Loglevel::Note),
            LoglevelFilter::Info => Ok(Loglevel::Info),
            LoglevelFilter::Debug => Ok(Loglevel::Debug),
            LoglevelFilter::Trace => Ok(Loglevel::Trace),
            LoglevelFilter::Off => Err(()),
        }
    }
}

impl From<Loglevel> for LoglevelFilter {
    fn from(level: Loglevel) -> LoglevelFilter {
        match level {
            Loglevel::Fatal => LoglevelFilter::Fatal,
            Loglevel::Error => LoglevelFilter::Error,
            Loglevel::Warn => LoglevelFilter::Warn,
            Loglevel::Note => LoglevelFilter::Note,
            Loglevel::Info => LoglevelFilter::Info,
            Loglevel::Debug => LoglevelFilter::Debug,
            Loglevel::Trace => LoglevelFilter::Trace,
        }
    }
}

/// Log record metadata.
///
/// The log metadata attached to a [`LogRecord`].
///
/// [`LogRecord`]: ./struct.LogRecord.html
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Loglevel of the log record.
    level: Loglevel,
    module_path: Option<String>,
    file: Option<String>,
    line: Option<u32>,
    timestamp: std::time::SystemTime,
    process: u32,
    thread: u64,
}

/// A log record.
///
/// A log record consists of some metadata and a payload.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogRecord {
    payload: String,
    metadata: Metadata,
    logger: String,
}

impl LogRecord {
    pub fn payload(&self) -> &str {
        &self.payload
    }
    pub fn level(&self) -> Loglevel {
        self.metadata.level
    }
    pub fn module_path(&self) -> Option<&str> {
        self.metadata.module_path.as_deref()
    }
    pub fn file(&self) -> Option<&str> {
        self.metadata.file.as_deref()
    }
    pub fn line(&self) -> Option<u32> {
        self.metadata.line
    }
    pub fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
    pub fn process(&self) -> u32 {
        self.metadata.process
    }
    pub fn thread(&self) -> u64 {
        self.metadata.thread
    }
    pub fn logger(&self) -> &str {
        self.logger.as_str()
    }
}

impl LogRecord {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn new(
        logger: impl Into<String>,
        payload: impl Into<String>,
        level: Loglevel,
        module_path: impl Into<String>,
        file: impl Into<String>,
        line: u32,
        process: u32,
        thread: u64,
    ) -> LogRecord {
        LogRecord {
            payload: payload.into(),
            metadata: Metadata {
                level,
                module_path: Some(module_path.into()),
                file: Some(file.into()),
                line: Some(line),
                timestamp: std::time::SystemTime::now(),
                process,
                thread,
            },
            logger: logger.into(),
        }
    }
}

impl fmt::Display for LogRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            humantime::format_rfc3339_seconds(self.metadata.timestamp)
        )?;
        write!(
            f,
            "{:<6}",
            format!(
                "+{}ms",
                self.metadata.timestamp.elapsed().unwrap().as_millis()
            )
        )?;
        write!(f, "{:>5} ", format!("{}", self.metadata.level))?;
        write!(
            f,
            "{:<32} ",
            format!(
                "{:>5}:{:<2} {} ",
                self.metadata.process, self.metadata.thread, self.logger,
            )
        )?;
        write!(f, "{}", self.payload)
    }
}

/// Update the thread-local loggers.
fn update(loggers: Option<Vec<Box<dyn Log>>>) -> error::Result<()> {
    LOGGERS.with(|x| {
        let mut x = x.try_borrow_mut().context(ErrorKind::LogError(
            "Unable to update thread-local loggers".to_string(),
        ))?;
        *x = loggers;
        Ok(())
    })
}

/// Initialize the thread-local loggers.
pub fn init(loggers: Vec<Box<dyn Log>>) -> error::Result<()> {
    update(Some(loggers))
}

/// Deinitialize the thread-local loggers.
pub fn deinit() -> error::Result<()> {
    update(None)
}

#[macro_export]
macro_rules! log {
    (? target: $target:expr, location: ($file:expr, $line:expr), $lvl:expr, $($arg:tt)+) => ({
        use $crate::common::log::_ref_thread_local::RefThreadLocal;
        $crate::common::log::LOGGERS.try_with(|loggers| {
            if let Some(ref loggers) = *loggers.borrow() {
                loggers.iter().for_each(|logger| {
                    if logger.enabled($lvl) {
                        logger.log(&$crate::common::log::LogRecord::new(
                            logger.name(),
                            format!($($arg)+),
                            $lvl,
                            $target,
                            $file,
                            $line,
                            *$crate::common::log::PID,
                            *$crate::common::log::TID.borrow()
                        ));
                    }
                });
                true
            } else {
                false
            }
        }).unwrap_or(false)
    });
    (target: $target:expr, location: ($file:expr, $line:expr), $lvl:expr, $($arg:tt)+) => (
        {
            $crate::log!(?
                target: module_path!(),
                location: ($file, $line),
                $lvl, $($arg)+
            );
        }
    );
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::log!(
            target: module_path!(),
            location: (file!(), line!()),
            $lvl, $($arg)+
        )
    );
    ($lvl:expr, $($arg:tt)+) => (
        $crate::log!(
            target: module_path!(),
            $lvl, $($arg)+
        )
    )
}

#[macro_export]
macro_rules! fatal {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Fatal, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Fatal, $($arg)+);
    )
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Error, $($arg)+);
    )
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Warn, $($arg)+);
    )
}

#[macro_export]
macro_rules! note {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Note, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Note, $($arg)+);
    )
}

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Info, $($arg)+);
    )
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Debug, $($arg)+);
    )
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log!(target: $target, $crate::common::log::Loglevel::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        $crate::log!($crate::common::log::Loglevel::Trace, $($arg)+);
    )
}

#[cfg(test)]
mod tests {
    use super::{LogRecord, Loglevel, LoglevelFilter};

    #[test]
    fn level_order() {
        assert!(Loglevel::Debug < Loglevel::Trace);
        assert!(Loglevel::Info < Loglevel::Debug);
        assert!(Loglevel::Note < Loglevel::Info);
        assert!(Loglevel::Warn < Loglevel::Note);
        assert!(Loglevel::Error < Loglevel::Warn);
        assert!(Loglevel::Fatal < Loglevel::Error);
        assert!(LoglevelFilter::Off < LoglevelFilter::from(Loglevel::Fatal));
    }

    #[test]
    fn level_colors() {
        let color: term::color::Color = Loglevel::Error.into();
        assert_eq!(color, term::color::RED);

        let color: term::color::Color = Loglevel::Note.into();
        assert_eq!(color, term::color::WHITE);
    }

    #[test]
    fn filter_to_level() {
        assert!(Loglevel::try_from(LoglevelFilter::Fatal).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Error).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Warn).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Note).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Info).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Debug).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Trace).is_ok());
        assert!(Loglevel::try_from(LoglevelFilter::Off).is_err());
    }

    #[test]
    fn log_record_debug_getters() {
        let record = LogRecord::new("", "", Loglevel::Debug, "path", "file", 1234u32, 1u32, 1u64);
        assert_eq!(record.module_path(), Some("path"));
        assert_eq!(record.file(), Some("file"));
        assert_eq!(record.line(), Some(1234u32));
    }

    #[test]
    fn display_record() {
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
        assert_eq!(
            &format!("{}", record).as_str()[24..],
            "  Trace     1:1  logger                  message"
        );
    }
}
