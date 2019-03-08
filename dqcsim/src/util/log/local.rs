use crate::util::log::LOGGER;

/// A ThreadLocalLogger instance implements `log::Log` to proxy log messages to
/// an initialized logger.
///
/// Normally the instance is created by the [`init`] function.
///
/// [`init`]: ./fn.init.html
pub struct ThreadLocalLogger;

impl log::Log for ThreadLocalLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        LOGGER.with(|logger| match *logger.borrow() {
            Some(ref logger) => logger.enabled(metadata),
            None => false,
        })
    }

    fn log(&self, record: &log::Record) {
        LOGGER.with(|logger| {
            if let Some(ref logger) = *logger.borrow() {
                logger.log(record)
            }
        });
    }

    fn flush(&self) {}
}
