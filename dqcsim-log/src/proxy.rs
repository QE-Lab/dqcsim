use crate::Sender;

/// A LogProxy is a logger implementation (log::Log) which sends log records using its Sender side of a Channel.
pub struct LogProxy<T: Sender> {
    level: log::LevelFilter,
    sender: T,
}

unsafe impl<T: Sender> Send for LogProxy<T> {}
unsafe impl<T: Sender> Sync for LogProxy<T> {}

impl<T: Sender> LogProxy<T> {
    pub fn new(sender: T, level_filter: Option<log::LevelFilter>) -> LogProxy<T> {
        LogProxy {
            sender,
            level: level_filter.unwrap_or(log::LevelFilter::Info),
        }
    }
    pub fn level(mut self, level_filter: log::LevelFilter) -> LogProxy<T> {
        self.level = level_filter;
        self
    }
}

impl<T: Sender + std::fmt::Debug> log::Log for LogProxy<T>
where
    for<'a> T::Item: From<&'a log::Record<'a>>,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.sender
                .send(T::Item::from(record))
                .expect("LogProxy failed to send record.");
        }
    }

    fn flush(&self) {}
}
