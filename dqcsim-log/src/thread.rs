use crate::{deinit, init, proxy::LogProxy, trace, Level, LevelFilter, Record};
use std::thread;
use term::stderr;

pub struct LogThread {
    sender: Option<crossbeam_channel::Sender<Record>>,
    handler: Option<thread::JoinHandle<Result<(), term::Error>>>,
}

/// Convert a Level to term::color::Color
fn level_to_color(level: Level) -> term::color::Color {
    match level {
        Level::Fatal => 9,
        Level::Error => 1,
        Level::Warn => 3,
        Level::Note => 7,
        Level::Info => 4,
        Level::Debug => 6,
        Level::Trace => 8,
    }
}

impl LogThread {
    pub fn get_sender(&self) -> Option<crossbeam_channel::Sender<Record>> {
        self.sender.clone()
    }
    pub fn spawn(level: LevelFilter) -> LogThread {
        // Create the log channel.
        let (sender, receiver): (_, crossbeam_channel::Receiver<Record>) =
            crossbeam_channel::unbounded();

        // Spawn the local channel log thread.
        let handler = thread::spawn(move || {
            let mut t = stderr().expect("Unable to wrap terminal");

            while let Ok(record) = receiver.recv() {
                t.reset()?;
                t.attr(term::Attr::Dim)?;
                write!(
                    t,
                    "{} ",
                    humantime::format_rfc3339_seconds(std::time::SystemTime::now())
                )?;
                t.reset()?;

                t.fg(level_to_color(record.level()))?;
                write!(t, "{:>5} ", format!("{}", record.level()))?;
                t.reset()?;

                if record.level() == Level::Trace {
                    t.fg(level_to_color(record.level()))?;
                }
                // if std::process::id() != record.process() {
                //     t.fg(record.process() % 6 + 1)?;
                // }
                writeln!(t, "{}", record)?;
                t.reset()?;
            }
            Ok(())
        });

        // Start a LogProxy for the current thread.
        init(LogProxy::boxed(sender.clone(), Some(level)), level).unwrap();

        LogThread {
            sender: Some(sender),
            handler: Some(handler),
        }
    }
}

/// Drops the sender side of the log channel and wait for the log thread to drop.
impl Drop for LogThread {
    fn drop(&mut self) {
        trace!("Shutting down logger thread");

        // Disconnect the LogProxy running in the main thread.
        deinit().unwrap();

        // Drop the owned sender side to disconnect the log channel.
        self.sender = None;

        // Wait for the logger thread to be dropped.
        self.handler
            .take()
            .expect("LogThread failed to start")
            .join()
            .expect("LogThread failed to terminate")
            .expect("LogThread failed");
    }
}
