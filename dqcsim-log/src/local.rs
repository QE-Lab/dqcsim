use std::cell::RefCell;

thread_local! {
    /// The thread local logger. Can be anything which implements log::Log.
    static LOGGER: RefCell<Option<Box<log::Log>>> = RefCell::new(None);
}

/// Initialize the logger.
/// This starts the ThreadLocalLogger which implements log::Log.
/// The ThreadLocalLogger forwards log records to the thread local logger.
pub fn init(level: Option<log::LevelFilter>) -> Result<(), log::SetLoggerError> {
    log::set_logger(&ThreadLocalLogger)
        .map(|()| log::set_max_level(level.unwrap_or(log::LevelFilter::Info)))
}

struct ThreadLocalLogger;

impl log::Log for ThreadLocalLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
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
