mod local;
mod proxy;
mod record;
mod sender;
mod stdio;
mod thread;

pub use local::*;
pub use proxy::*;
pub use record::*;
pub use sender::*;
pub use stdio::*;
pub use thread::*;

use failure::Error;
use ipc_channel::ipc;
use std::cell::RefCell;

thread_local! {
    /// The thread local logger. Can be set to anything which implements `log::Log`.
    static LOGGER: RefCell<Option<Box<log::Log>>> = RefCell::new(None);
}

/// Initialize the thread local logger system.
///
/// This creates an [`ThreadLocalLogger`] instance which implements `log::Log` to forward
/// log records to a [`ThreadLocalLogger`]. If no thread local logger is set using
/// the [`set_thread_logger`] function, log records are discarded.
///
/// The init function takes an optional `log::LevelFilter` value to set the global
/// maximum log level. If no value is provided, `log::LevelFilter::Info` is used.
///
/// [`ThreadLocalLogger`]: ./struct.ThreadLocalLogger.html
/// [`set_thread_logger`]: ./fn.set_thread_logger.html
pub fn init(level: Option<log::LevelFilter>) -> Result<(), log::SetLoggerError> {
    log::set_logger(&ThreadLocalLogger)
        .map(|_| log::set_max_level(level.unwrap_or(log::LevelFilter::Info)))
}

/// Connects to a [`LogThread`] receiver server via an IPC channel.
///
/// This also starts the [`ThreadLocalLogger`] and connects to the log channel.
///
/// In addition to the optinal `log::LevelFilter` value to set the global maximum
/// log level, the connect functions takes a server address to connect to.
/// Normally the server address is provided by the plugin controller thread
/// when starting a child process which should connect to the log thread.
///
/// The function attempts to connect to the server, creates a new IPC channel for
/// log records and sends the receiver side to the server. The sender side can
/// be used to send log records to the server, which forwards these log records to
/// the log thread.
/// The connect function also initializes the thread local logger system and creates
/// a proxy instance which gets the sender side of the IPC channel.
///
/// [`LogThread`]: ./struct.LogThread.html
/// [`ThreadLocalLogger`]: ./struct.ThreadLocalLogger.html
pub fn connect(server: impl Into<String>, level: Option<log::LevelFilter>) -> Result<(), Error> {
    // Connect to the server.
    let connect = ipc::IpcSender::connect(server.into())?;

    // Create log channel
    let (tx, rx): (ipc::IpcSender<Record>, _) = ipc::channel()?;
    // Send receiver to host.
    connect.send(rx)?;

    // Initialize thread local logger.
    init(level).expect("Unable to set thread local logger");

    // Setup log proxy.
    set_thread_logger(LogProxy::boxed(tx, level));

    log::trace!("Connected to log channel via ipc");
    Ok(())
}

/// Sets the thread local logger.
///
/// Normally this function is called once after initialization of the
/// [`ThreadLocalLogger`] to set the thread local logger.
///
/// [`ThreadLocalLogger`]: ./struct.ThreadLocalLogger.html
pub fn set_thread_logger(log: Box<log::Log>) {
    LOGGER.with(|logger| {
        *logger.borrow_mut() = Some(log);
    })
}

/// Drops the thread local logger.
///
/// Normally the thread logger gets dropped automatically, however, to make
/// sure the log thread gets dropped, all senders need to be dropped. The sender
/// used in the log thread needs to be dropped explicitly.
pub fn drop_thread_logger() {
    LOGGER.with(|logger| {
        *logger.borrow_mut() = None;
    })
}
