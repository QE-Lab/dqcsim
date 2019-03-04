mod local;
mod proxy;
mod record;
mod sender;
mod thread;

pub use proxy::*;
pub use record::*;
pub use sender::*;
pub use thread::*;

use crate::local::ThreadLocalLogger;
use ipc_channel::ipc;
use std::{cell::RefCell, error::Error};

thread_local! {
    /// The thread local logger. Can be anything which implements log::Log.
    static LOGGER: RefCell<Option<Box<log::Log>>> = RefCell::new(None);
}

/// Initialize the logger.
/// This starts the ThreadLocalLogger which implements log::Log.
/// The ThreadLocalLogger forwards log records to the thread local logger.
pub fn init(level: Option<log::LevelFilter>) -> Result<(), log::SetLoggerError> {
    log::set_logger(&ThreadLocalLogger)
        .map(|_| log::set_max_level(level.unwrap_or(log::LevelFilter::Info)))
}

/// Connects to a logger thread receiver server via an IPC channel.
/// This starts the ThreadLocalLogger and connects to the log channel.
pub fn connect<'a>(
    server: impl Into<String>,
    level: Option<log::LevelFilter>,
) -> Result<(), Box<dyn Error>> {
    // Connect to the server.
    let connect = ipc::IpcSender::connect(server.into())?;

    // Create log channel
    let (tx, rx): (ipc::IpcSender<Record>, _) = ipc::channel()?;

    // Send receiver to host.
    connect.send(rx)?;

    // Initialize thread local logger.
    init(level).expect("Unable to set thread local logger.");

    // Setup log proxy.
    set_thread_logger(Box::new(LogProxy::new(tx, Some(log::LevelFilter::Trace))));

    log::trace!("Connected to log channel via ipc");
    Ok(())
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
