use crate::Record;
use crossbeam_channel::Sender;

/// A LogProxy is a logger implementation (log::Log) which sends log records to a LogThread.
pub struct LogProxy {
    level: log::LevelFilter,
    sender: Sender<Record>,
}

impl LogProxy {
    pub fn new(sender: Sender<Record>, level_filter: Option<log::LevelFilter>) -> LogProxy {
        LogProxy {
            sender,
            level: level_filter.unwrap_or_else(log::LevelFilter::max),
        }
    }
    pub fn level(mut self, level_filter: log::LevelFilter) -> LogProxy {
        self.level = level_filter;
        self
    }
}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        self.sender
            .try_send(Record::from(record))
            .expect("LogProxy failed.");
    }

    fn flush(&self) {}
}
