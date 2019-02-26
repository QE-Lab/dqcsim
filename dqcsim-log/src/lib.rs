use crossbeam_channel::{Receiver, Sender};
use log::{log, Level, LevelFilter, Metadata, SetLoggerError};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt, thread::JoinHandle};

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
            // Default to logger
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
    // module_path: String,
    // file: String,
    // line: u32,
}

impl<'a> From<&log::Record<'a>> for Record {
    fn from(log: &log::Record<'a>) -> Record {
        Record {
            level: log.level(),
            args: log.args().to_string(),
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

impl LogThread {
    /// Starts a new log thread.
    /// Also starts a LogProxy for the thread starting the log thread.
    pub fn new() -> LogThread {
        // Create the log channel.
        let (sender, receiver): (_, Receiver<Record>) = crossbeam_channel::unbounded();

        // Spawn the log thread.
        let handler = std::thread::spawn(move || loop {
            match receiver.recv() {
                Ok(record) => {
                    println!("[{}]: {}", record.level, record.args);
                }
                Err(_) => {
                    println!("Log channel disconnected: dropping log thread");
                    break;
                }
            }
        });

        // Start a LogProxy for the current thread.
        set_thread_logger(Box::new(LogProxy {
            sender: Some(sender.clone()),
        }));

        LogThread {
            sender: Some(sender),
            handler: Some(handler),
        }
    }

    /// Returns a copy of the sender side of the log channel.
    pub fn get_sender(&self) -> Option<Sender<Record>> {
        self.sender.clone()
    }
}

/// When the LogThread goes out of scope:
/// Drops the sender side of the log channel and wait for the log thread to drop.
impl Drop for LogThread {
    fn drop(&mut self) {
        // Disconnect the LogProxy running in the main thread.
        drop_thread_logger();

        // Drop the owned sender side to disconnect the log channel.
        self.sender = None;

        // Wait for the thread to be dropped.
        self.handler
            .take()
            .expect("LogThread failed.")
            .join()
            .expect("LogThread failed.");
    }
}

/// A LogProxy is a logger implementation (log::Log) which sends log records to a LogThread.
pub struct LogProxy {
    pub sender: Option<Sender<Record>>,
}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Error
        true
    }

    fn log(&self, record: &log::Record) {
        match self.sender {
            Some(ref sender) => sender
                .try_send(Record::from(record))
                // record.args().to_string())
                .expect("LogProxy failed."),
            None => log::logger().log(record),
        };
    }

    fn flush(&self) {}
}
