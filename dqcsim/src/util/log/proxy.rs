use crate::util::log::Sender;

/// A [`LogProxy`] is a logger implementation (`log::Log`) which sends log records
/// using its Sender side of a Channel.
///
/// [`LogProxy`]: ./struct.LogProxy.html
pub struct LogProxy<T: Sender> {
    level: log::LevelFilter,
    sender: T,
}

/// The `log::Log` trait requires the type implementing the trait to be Send + Sync.
/// Not all [`LogProxy`] Sender types are Send + Sync (i.e. ipc_channel).
/// However as the intended use of the [`LogProxy`] is in a [`ThreadLocalLogger`] this
/// is not a problem.
///
/// [`LogProxy`]: ./struct.LogProxy.html
/// [`ThreadLocalLogger`]: ./struct.ThreadLocalLogger.html
unsafe impl<T: Sender> Send for LogProxy<T> {}
unsafe impl<T: Sender> Sync for LogProxy<T> {}

impl<T: Sender> LogProxy<T> {
    fn new(sender: T, level_filter: Option<log::LevelFilter>) -> LogProxy<T> {
        LogProxy {
            sender,
            level: level_filter.unwrap_or(log::LevelFilter::Info),
        }
    }

    /// Return a new boxed LogProxy for the provided sender and level.
    pub fn boxed(sender: T, level_filter: Option<log::LevelFilter>) -> Box<LogProxy<T>> {
        Box::new(LogProxy::new(sender, level_filter))
    }

    /// Modify the level of the LogProxy.
    pub fn level(mut self, level_filter: log::LevelFilter) -> LogProxy<T> {
        self.level = level_filter;
        self
    }
}

impl<T: Sender> log::Log for LogProxy<T>
where
    for<'a> T::Item: From<&'a log::Record<'a>>,
{
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // Use the owned sender to forward the log records.
            self.sender
                .send(T::Item::from(record))
                .expect("LogProxy failed to send record");
        }
    }

    fn flush(&self) {}
}
