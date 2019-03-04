use crate::LOGGER;

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
