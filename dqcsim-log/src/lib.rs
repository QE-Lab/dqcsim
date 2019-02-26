use crossbeam_channel::{Receiver, Sender};
use log::{trace, Level, LevelFilter, Metadata, SetLoggerError};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt, thread::JoinHandle};
use term::stderr;

thread_local! {
    /// The thread local logger. Can be anything which implements log::Log.
    static LOGGER: RefCell<Option<Box<log::Log>>> = RefCell::new(None);
}

/// Initialize the logger.
/// This starts the ThreadLocalLogger which implements log::Log.
/// The ThreadLocalLogger forwards log records to the thread local logger.
pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&ThreadLocalLogger).map(|()| log::set_max_level(level))
}

struct ThreadLocalLogger;

impl log::Log for ThreadLocalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        LOGGER.with(|logger| match *logger.borrow() {
            Some(ref logger) => logger.enabled(metadata),
            None => false,
        })
    }

    fn log(&self, record: &log::Record) {
        LOGGER.with(|logger| match *logger.borrow_mut() {
            Some(ref mut logger) => logger.log(record),
            // Defaults to initialized logger (or log::NopLogger if uninitialized)
            None => log::logger().log(record),
        });
    }

    fn flush(&self) {}
}

/// Sets the thread local logger.
pub fn set_thread_logger(log: Box<log::Log>) {
    LOGGER.with(|logger| {
        *logger.borrow_mut() = Some(log);
    })
}

/// Drops the thread local logger.
pub fn drop_thread_logger() {
    LOGGER.with(|logger| {
        *logger.borrow_mut() = None;
    })
}

// #[derive(Serialize, Deserialize)]
// struct Metadata {
//     level: Level,
//     target: String,
// }

#[derive(Serialize, Deserialize)]
pub struct Record {
    level: Level,
    args: String,
    target: String,
    // module_path: String,
    // file: String,
    // line: u32,
}

impl<'a> From<&log::Record<'a>> for Record {
    fn from(log: &log::Record<'a>) -> Record {
        Record {
            level: log.level(),
            args: log.args().to_string(),
            target: std::thread::current()
                .name()
                .unwrap_or_default()
                .to_string(),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.args)
    }
}

/// A thread dedicated to logging.
/// The log thread provides producers with a copy of the sender side of the log channel.
/// Producers can use this sender side of the log channel to forward their log records.
pub struct LogThread {
    sender: Option<Sender<Record>>,
    handler: Option<JoinHandle<()>>,
}

impl Default for LogThread {
    fn default() -> LogThread {
        LogThread::new()
    }
}

impl LogThread {
    /// Starts a new log thread.
    /// Also starts a LogProxy for the thread starting the log thread.
    pub fn new() -> LogThread {
        // Create the log channel.
        let (sender, receiver): (_, Receiver<Record>) = crossbeam_channel::unbounded();

        // Spawn the log thread.
        let handler = std::thread::spawn(move || {
            let mut t = stderr().expect("Unable to wrap terminal.");
            while let Ok(record) = receiver.recv() {
                writeln!(
                    t,
                    "[{} {:<5} {}] {}",
                    humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                    record.level,
                    record.target,
                    record
                )
                .unwrap();
            }
        });

        // Start a LogProxy for the current thread.
        set_thread_logger(Box::new(LogProxy::new(sender.clone(), None)));

        LogThread {
            sender: Some(sender),
            handler: Some(handler),
        }
    }

    /// Returns a copy of the sender side of the log channel.
    /// Use this sender side of the log channel to pass Records to the logger thread.
    pub fn get_sender(&self) -> Option<Sender<Record>> {
        self.sender.clone()
    }
}

/// When the LogThread goes out of scope:
/// Drops the sender side of the log channel and wait for the log thread to drop.
impl Drop for LogThread {
    fn drop(&mut self) {
        trace!("Shutting down logger thread.");

        // Disconnect the LogProxy running in the main thread.
        drop_thread_logger();

        // Drop the owned sender side to disconnect the log channel.
        self.sender = None;

        // Wait for the logger thread to be dropped.
        self.handler
            .take()
            .expect("LogThread failed.")
            .join()
            .expect("LogThread failed.");
    }
}

/// A LogProxy is a logger implementation (log::Log) which sends log records to a LogThread.
pub struct LogProxy {
    level: log::LevelFilter,
    sender: Sender<Record>,
}

impl LogProxy {
    pub fn new(sender: Sender<Record>, level_filter: Option<log::LevelFilter>) -> LogProxy {
        LogProxy {
            sender,
            level: level_filter.unwrap_or(LevelFilter::max()),
        }
    }
    pub fn level(mut self, level_filter: log::LevelFilter) -> LogProxy {
        self.level = level_filter;
        self
    }
}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        self.sender
            .try_send(Record::from(record))
            .expect("LogProxy failed.");
    }

    fn flush(&self) {}
}
