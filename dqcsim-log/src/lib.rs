use std::sync::Arc;
use crossbeam_channel::{Receiver, Sender};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, thread::JoinHandle};

thread_local! {
    static LOGGER: RefCell<Option<Box<log::Log>>> = RefCell::new(None);
}

pub fn set_thread_logger(log: Box<log::Log>) {
    LOGGER.with(|logger| {
        *logger.borrow_mut() = Some(log);
    })
}

pub fn drop_thread_logger() {
    LOGGER.with(|logger| {
        *logger.borrow_mut() = None;
    })
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(ThreadLocalLogger))
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

// #[derive(Serialize, Deserialize)]
// pub struct LogMetadata {
//     level: Level,
//     target: String,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct LogRecord {
//     metadata: LogMetadata,
//     args: String,
//     module_path: String,
//     file: String,
//     line: u32,
// }

pub struct LogThread {
    sender: Sender<String>,
    handler: JoinHandle<()>,
}

impl LogThread {
    pub fn new() -> LogThread {
        let (sender, receiver): (_, Receiver<String>) = crossbeam_channel::unbounded();
        let handler = std::thread::spawn(move || loop {
            match receiver.recv() {
                Ok(record) => {
                    println!("[logger]: {}", record);
                },
                Err(_) => {
                    println!("stopping log thread");
                    break;
                }
            }
        });
        LogThread {
            sender,
            handler,
        }
    }
    pub fn get_sender(&self) -> Sender<String> {
        self.sender.clone()
    }
    pub fn wait(mut self) {
        println!("waiting for log thread");
        drop(self.receiver);
        self.handler.join().unwrap();
        ()
    }
}

struct ThreadLocalLogger;

impl log::Log for ThreadLocalLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        LOGGER.with(|logger| {
            match *logger.borrow() {
                Some(ref logger) => logger.enabled(metadata),
                None => false
            }
        })
    }

    fn log(&self, record: &Record) {
        LOGGER.with(|logger| {
            match *logger.borrow_mut() {
                Some(ref mut logger) => logger.log(record),
                None => {
                    println!("Please set a logger.")
                }
            }
        });
    }

    fn flush(&self) {}
}

pub struct LogProxy {
    pub sender: Sender<String>,
}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Error
        true
    }

    fn log(&self, record: &Record) {
        // println!("[{:?}] {:?}", std::thread::current().id(), record);
        self.sender
            .send(record.args().to_string())
            .unwrap()
            //  {
            //     metadata: LogMetadata {
            //         level: record.metadata().level(),
            //         target: record.metadata().target().to_owned(),
            //     },
            //     args: record.args().to_owned().to_string(),
            //     module_path: record.module_path().unwrap_or_default().to_owned(),
            //     file: record.file().unwrap_or_default().to_owned(),
            //     line: record.line().unwrap_or_default().to_owned(),
            // })
            // .unwrap();
        // self.tx.send(record.clone());
        // if self.enabled(record.metadata()) {
        // println!("{} - {}", record.level(), record.args());
        // }
    }

    fn flush(&self) {}
}
