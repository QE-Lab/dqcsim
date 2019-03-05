use crate::log::{drop_thread_logger, set_thread_logger, LogProxy, Record};
use std::thread::JoinHandle;
use term::stderr;

/// A thread dedicated to logging.
/// The log thread provides log producers with a copy of the sender side of the log channel.
/// Producers can use this sender side of the log channel to forward their log records.
pub struct LogThread {
    sender: Option<crossbeam_channel::Sender<Record>>,
    handler: Option<JoinHandle<Result<(), term::Error>>>,
}

fn level_to_color(level: log::Level) -> term::color::Color {
    match level {
        log::Level::Error => 1,
        log::Level::Warn => 3,
        log::Level::Info => 2,
        log::Level::Debug => 6,
        log::Level::Trace => 4,
    }
}

impl LogThread {
    /// Starts a new log thread.
    /// Also starts a LogProxy for the thread starting the log thread.
    pub fn new(level_filter: Option<log::LevelFilter>) -> LogThread {
        // Create the log channel.
        let (sender, receiver): (_, crossbeam_channel::Receiver<Record>) =
            crossbeam_channel::unbounded();

        // Spawn the local channel log thread.
        let handler = std::thread::spawn(move || {
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

                t.attr(term::Attr::Standout(true))?;
                t.fg(level_to_color(*record.level()))?;
                write!(t, "{:5}", record.level())?;
                t.reset()?;

                t.attr(term::Attr::Dim)?;
                let target = record.target();
                if target.len() >= 15 {
                    write!(t, " {:15}", unsafe { target.get_unchecked(0..14) })?;
                } else {
                    write!(t, " {:15}", target)?;
                }
                t.reset()?;

                if std::process::id() != record.process() {
                    t.fg(record.process() % 6 + 1)?;
                } else if record.thread() == "main" {
                    // DQCsim main thread log messages are bold
                    t.attr(term::Attr::Bold)?;
                }
                writeln!(t, "{}", record)?;
                t.reset()?;
            }
            Ok(())
        });

        // Start a LogProxy for the current thread.
        set_thread_logger(Box::new(LogProxy::new(sender.clone(), level_filter)));

        LogThread {
            sender: Some(sender),
            handler: Some(handler),
        }
    }

    /// Returns a copy of the sender side of the log channel.
    /// Use this sender side of the log channel to pass Records to the logger thread.
    pub fn get_sender(&self) -> Option<crossbeam_channel::Sender<Record>> {
        self.sender.clone()
    }
}

/// When the LogThread goes out of scope:
/// Drops the sender side of the log channel and wait for the log thread to drop.
impl Drop for LogThread {
    fn drop(&mut self) {
        log::trace!("Shutting down logger thread.");

        // Disconnect the LogProxy running in the main thread.
        drop_thread_logger();

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
