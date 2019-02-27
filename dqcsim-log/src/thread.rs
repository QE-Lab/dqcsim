use crate::{drop_thread_logger, set_thread_logger, LogProxy, Record};
use crossbeam_channel::{Receiver, Sender};
use std::thread::JoinHandle;
use term::stderr;

/// A thread dedicated to logging.
/// The log thread provides producers with a copy of the sender side of the log channel.
/// Producers can use this sender side of the log channel to forward their log records.
pub struct LogThread {
    sender: Option<Sender<Record>>,
    handler: Option<JoinHandle<Result<(), term::Error>>>,
}

impl Default for LogThread {
    fn default() -> LogThread {
        LogThread::new(Some(log::LevelFilter::Info))
    }
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
        let (sender, receiver): (_, Receiver<Record>) = crossbeam_channel::unbounded();

        // Spawn the log thread.
        let handler = std::thread::spawn(move || {
            let mut t = stderr().expect("Unable to wrap terminal.");
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
                if target.len() >= 8 {
                    write!(t, " {:8}", unsafe { target.get_unchecked(0..7) })?;
                } else {
                    write!(t, " {:8}", target)?;
                }
                t.reset()?;

                // Main thread log messages are bold
                if record.target() == "main" {
                    t.attr(term::Attr::Bold)?;
                }
                match record.target() {
                    "main" => t.attr(term::Attr::Bold)?,
                    "backend" => t.fg(term::color::GREEN)?,
                    "frontend" => t.fg(term::color::BLUE)?,
                    _ => (),
                }
                writeln!(t, "{}", record)?;
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
    pub fn get_sender(&self) -> Option<Sender<Record>> {
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
            .expect("LogThread failed.")
            .join()
            .expect("LogThread failed.")
            .expect("LogThread failed.");
    }
}
