use crossbeam_channel::{Receiver, Sender};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use serde::{Deserialize, Serialize};
use std::thread::JoinHandle;

// thread_local! {
//     static LOGGER: LogProxy = LogProxy::new();
// }

pub fn init(sender: Sender<LogRecord>) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(LogProxy { sender }))
        .map(|()| log::set_max_level(LevelFilter::Trace))
    // LOGGER.set_sender(sender);
    // log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

#[derive(Serialize, Deserialize)]
pub struct LogMetadata {
    level: Level,
    target: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogRecord {
    metadata: LogMetadata,
    args: String,
    module_path: String,
    file: String,
    line: u32,
}

pub struct LogThread {
    sender: Option<Sender<LogRecord>>,
    handler: JoinHandle<()>,
}

impl LogThread {
    pub fn new() -> LogThread {
        let (sender, receiver): (_, Receiver<LogRecord>) = crossbeam_channel::unbounded();
        let handler = std::thread::spawn(move || loop {
            match receiver.recv() {
                Ok(record) => println!("[{}] - {}", record.metadata.level, record.args),
                Err(_) => {
                    println!("stopping log thread");
                    break;
                }
            }
        });
        LogThread {
            sender: Some(sender),
            handler,
        }
    }
    pub fn get_sender(&self) -> Option<Sender<LogRecord>> {
        self.sender.clone()
    }
    pub fn wait(mut self) {
        println!("waiting for log thread");
        self.sender = None;
        self.handler.join().unwrap();
        ()
    }
}

pub struct LogProxy {
    sender: Sender<LogRecord>,
}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Error
        true
    }

    fn log(&self, record: &Record) {
        println!("tid logging record: {:?}", std::thread::current().id());
        self.sender
            .send(LogRecord {
                metadata: LogMetadata {
                    level: record.metadata().level(),
                    target: record.metadata().target().to_owned(),
                },
                args: record.args().to_owned().to_string(),
                module_path: record.module_path().unwrap_or_default().to_owned(),
                file: record.file().unwrap_or_default().to_owned(),
                line: record.line().unwrap_or_default().to_owned(),
            })
            .unwrap();
        // self.tx.send(record.clone());
        // if self.enabled(record.metadata()) {
        // println!("{} - {}", record.level(), record.args());
        // }
    }

    fn flush(&self) {}
}
