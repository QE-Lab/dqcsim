use crate::{deinit, init, proxy::LogProxy, trace, Level, LevelFilter, Record, PID};
use failure::Error;
use std::thread;
use term::stderr;

#[derive(Debug)]
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
    pub fn spawn(level: LevelFilter) -> Result<LogThread, Error> {
        // Create the log channel.
        let (sender, receiver): (_, crossbeam_channel::Receiver<Record>) =
            crossbeam_channel::unbounded();

        // Spawn the local channel log thread.
        let handler = thread::spawn(move || {
            let mut t = stderr().expect("Unable to wrap terminal");

            let supports_dim = t.supports_attr(term::Attr::Dim);
            let supports_colors = t.supports_attr(term::Attr::ForegroundColor(9));

            let trace = level >= LevelFilter::Trace;
            let debug = level >= LevelFilter::Debug;

            while let Ok(record) = receiver.recv() {
                let color = level_to_color(record.level());

                // Timestamp
                t.reset()?;
                if supports_dim {
                    t.attr(term::Attr::Dim)?;
                }
                write!(
                    t,
                    "{} ",
                    humantime::format_rfc3339_seconds(record.timestamp()),
                )?;
                t.reset()?;

                // Delay
                if debug {
                    if supports_colors {
                        t.fg(8)?;
                    }
                    write!(
                        t,
                        "{:<6}",
                        format!("+{}ms", record.timestamp().elapsed().unwrap().as_millis())
                    )?;
                    t.reset()?;
                }

                // Record level
                if supports_colors {
                    t.fg(color)?;
                }
                write!(t, "{:>5} ", format!("{}", record.level()))?;
                t.reset()?;

                // Identifier
                if supports_colors && *PID != record.process() {
                    t.fg(record.process() % 7 + 1)?;
                }
                if supports_dim {
                    t.attr(term::Attr::Dim)?;
                }
                if trace {
                    // With process + thread identifier
                    write!(
                        t,
                        "{:<35} ",
                        format!(
                            "{:>5}:{:<2} {} ",
                            record.process(),
                            record.thread(),
                            record.module_path().unwrap(),
                        )
                    )?;
                } else {
                    write!(t, "{:<28} ", record.module_path().unwrap())?;
                }
                t.reset()?;

                // Record
                if supports_colors && record.level() == Level::Trace {
                    t.fg(color)?;
                }
                writeln!(t, "{}", record)?;
                t.reset()?;
            }
            // Trace log thread down
            if trace {
                if supports_colors {
                    if t.supports_attr(term::Attr::Standout(true)) {
                        t.attr(term::Attr::Standout(true))?;
                    }
                    t.fg(term::color::BRIGHT_BLACK)?;
                }
                writeln!(t, "$")?;
            }
            t.reset()?;
            Ok(())
        });

        // Start a LogProxy for the current thread.
        init(LogProxy::boxed(sender.clone()), level)?;

        Ok(LogThread {
            sender: Some(sender),
            handler: Some(handler),
        })
    }
    pub fn get_sender(&self) -> Option<crossbeam_channel::Sender<Record>> {
        self.sender.clone()
    }
}

/// Drops the sender side of the log channel and wait for the log thread to drop.
impl Drop for LogThread {
    fn drop(&mut self) {
        trace!("Shutting down logger thread");

        // Disconnect the LogProxy running in the main thread.
        deinit().expect("Failed to deinitialize thread-local logger");

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
