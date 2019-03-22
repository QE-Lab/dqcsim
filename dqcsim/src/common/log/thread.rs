//! A log thread implementation.

use crate::{
    common::{
        error::{oe_log_err, Result},
        log::{
            callback::LogCallback, deinit, init, proxy::LogProxy, tee_file::TeeFile, Log, Loglevel,
            LoglevelFilter, Record, PID,
        },
    },
    trace,
};
use std::thread;
use term::stderr;

#[derive(Debug)]
pub struct LogThread {
    sender: Option<crossbeam_channel::Sender<Record>>,
    handler: Option<thread::JoinHandle<Result<()>>>,
}

impl LogThread {
    /// Spawn a [`LogThread`].
    ///
    /// Returns [`LogThread`] instance if succesful. Also spawns a [`LogProxy`] in the current thread with the provided [`name`] and [`proxy_level`] as [`LogLevelFilter`].
    ///
    /// Output to Standard Error can be enabled by settings the [`stderr_level`] above [`LoglevelFilter::Off`].
    /// Output by invocatio of a callback function can be enabled by passing a
    /// [`LogCallback`] to [`callback`].
    pub fn spawn(
        name: impl Into<String>,
        proxy_level: LoglevelFilter,
        stderr_level: LoglevelFilter,
        callback: Option<LogCallback>,
        tee_files: Vec<TeeFile>,
    ) -> Result<LogThread> {
        // Create the log channel.
        let (sender, receiver): (_, crossbeam_channel::Receiver<Record>) =
            crossbeam_channel::unbounded();

        // Spawn the local channel log thread.
        let handler = thread::spawn(move || {
            let mut t = if stderr_level > LoglevelFilter::Off {
                Some(stderr().ok_or_else(oe_log_err("Failed to acquire stderr"))?)
            } else {
                None
            };

            let supports_dim = t.is_some() && t.as_ref().unwrap().supports_attr(term::Attr::Dim);
            let supports_colors = t.is_some()
                && t.as_ref()
                    .unwrap()
                    .supports_attr(term::Attr::ForegroundColor(9));

            let trace = stderr_level >= LoglevelFilter::Trace;
            let debug = stderr_level >= LoglevelFilter::Debug;

            while let Ok(record) = receiver.recv() {
                let level = LoglevelFilter::from(record.level());

                // Callback
                if let Some(callback) = &callback {
                    callback.log(&record);
                }

                // Tee files
                tee_files
                    .iter()
                    .filter(|tf| tf.enabled(record.level()))
                    .for_each(|tf| tf.log(&record));

                // Standard Error
                if level <= stderr_level {
                    let t = t.as_mut().unwrap();
                    let color: term::color::Color = record.level().into();

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
                            "{:<32} ",
                            format!(
                                "{:>5}:{:<2} {} ",
                                record.process(),
                                record.thread(),
                                record.logger(),
                            )
                        )?;
                    } else {
                        write!(t, "{:<25} ", record.logger())?;
                    }
                    t.reset()?;

                    // Record
                    if supports_colors && record.level() == Loglevel::Trace {
                        t.fg(color)?;
                    }
                    writeln!(t, "{}", record)?;
                    t.reset()?;
                }
            }
            // Trace log thread down
            if trace {
                let t = t.as_mut().unwrap();
                if supports_colors {
                    if t.supports_attr(term::Attr::Standout(true)) {
                        t.attr(term::Attr::Standout(true))?;
                    }
                    t.fg(term::color::BRIGHT_BLACK)?;
                }
                writeln!(t, "$")?;
            }
            if t.is_some() {
                t.unwrap().reset()?;
            }
            Ok(())
        });

        // Start a LogProxy for the current thread.
        init(vec![LogProxy::boxed(name, proxy_level, sender.clone())])?;
        trace!("LogThread started");

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
        trace!("Dropping LogThread");

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
