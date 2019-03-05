use crate::util::log::LOGGER;

/// A ThreadLocalLogger instance implements log::Log to proxy log messages to
/// an initialized logger.
///
/// Normally the instance is created by the `[dqcsim::util::log::init]` function.
pub struct ThreadLocalLogger;

impl log::Log for ThreadLocalLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        LOGGER.with(|logger| match *logger.borrow() {
            Some(ref logger) => logger.enabled(metadata),
            None => false,
        })
    }

    fn log(&self, record: &log::Record) {
        LOGGER.with(|logger| match *logger.borrow() {
            Some(ref logger) => logger.log(record),
            None => {} // no thread-local logger set.
        });
    }

    fn flush(&self) {}
}
