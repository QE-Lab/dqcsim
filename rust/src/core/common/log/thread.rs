//! A log thread implementation.

use crate::{
    common::{
        error::{err, Result},
        log::{
            callback::LogCallback,
            deinit, init,
            proxy::LogProxy,
            tee_file::{TeeFile, TeeFileConfiguration},
            Log, LogRecord, Loglevel, LoglevelFilter, PID,
        },
    },
    trace,
};
use std::thread;
use term::stderr;

#[derive(Debug)]
pub struct LogThread {
    sender: Option<crossbeam_channel::Sender<LogRecord>>,
    ipc_sender: Option<ipc_channel::ipc::IpcSender<LogRecord>>,
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
        tee_files: Vec<TeeFileConfiguration>,
    ) -> Result<LogThread> {
        // Create the log channel.
        let (sender, receiver): (_, crossbeam_channel::Receiver<LogRecord>) =
            crossbeam_channel::unbounded();

        // Create the IPC log channel.
        let (ipc_sender, ipc_receiver) = ipc_channel::ipc::channel()?;

        // Spawn the local channel log thread.
        let handler = thread::spawn(move || {
            let mut t = if stderr_level > LoglevelFilter::Off {
                // This may return None which results in no logging to stderr
                stderr()
            } else {
                None
            };

            let supports_dim = t.is_some() && t.as_ref().unwrap().supports_attr(term::Attr::Dim);
            let supports_colors = t.is_some()
                && t.as_ref()
                    .unwrap()
                    .supports_attr(term::Attr::ForegroundColor(9));

            let trace = t.is_some() && stderr_level >= LoglevelFilter::Trace;
            let debug = t.is_some() && stderr_level >= LoglevelFilter::Debug;

            let tee_files: Vec<TeeFile> = tee_files
                .into_iter()
                .map(TeeFile::new)
                .collect::<Result<Vec<_>>>()
                .or_else(|e| err(e.to_string()))?;

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
                if t.is_some() && level <= stderr_level {
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

                    // LogRecord level
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

                    // LogRecord
                    if supports_colors && record.level() == Loglevel::Trace {
                        t.fg(color)?;
                    }
                    writeln!(t, "{}", record.payload())?;
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
            if let Some(mut term) = t {
                term.reset()?;
            }
            Ok(())
        });

        // Start the IPC proxy.
        ipc_channel::router::ROUTER
            .route_ipc_receiver_to_crossbeam_sender(ipc_receiver, sender.clone());

        // Start a LogProxy for the current thread.
        init(vec![LogProxy::boxed(name, proxy_level, sender.clone())])?;
        trace!("LogThread started");

        Ok(LogThread {
            sender: Some(sender),
            ipc_sender: Some(ipc_sender),
            handler: Some(handler),
        })
    }
    pub fn get_sender(&self) -> crossbeam_channel::Sender<LogRecord> {
        self.sender.clone().unwrap()
    }

    pub fn get_ipc_sender(&self) -> ipc_channel::ipc::IpcSender<LogRecord> {
        self.ipc_sender.clone().unwrap()
    }
}

/// Drops the sender side of the log channel and wait for the log thread to drop.
impl Drop for LogThread {
    fn drop(&mut self) {
        trace!("Dropping LogThread");

        // Disconnect the LogProxy running in the main thread.
        deinit().expect("Failed to deinitialize thread-local logger");

        // Drop the owned senders to disconnect the log channels.
        self.sender = None;
        self.ipc_sender = None;

        // Wait for the logger thread to be dropped.
        self.handler
            .take()
            .expect("LogThread failed to start")
            .join()
            .expect("LogThread failed to terminate")
            .expect("LogThread failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let _lt = LogThread::spawn(
            "name",
            LoglevelFilter::Debug,
            LoglevelFilter::Error,
            None,
            vec![],
        )
        .unwrap();
        #[cfg(target_os = "macos")]
        assert_eq!(&format!("{:?}", _lt).as_str()[..100], "LogThread { sender: Some(Sender { .. }), ipc_sender: Some(IpcSender { os_sender: OsIpcSender { port:");
        #[cfg(target_os = "linux")]
        assert_eq!(&format!("{:?}", _lt).as_str()[..98], "LogThread { sender: Some(Sender { .. }), ipc_sender: Some(IpcSender { os_sender: OsIpcSender { fd:");
    }
}
