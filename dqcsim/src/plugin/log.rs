//! Utility function to setup logging for a Plugin instance.

use crate::{
    common::{
        error::Result,
        log::{init, proxy::LogProxy, tee_file::TeeFile, Log, LogRecord},
    },
    host::configuration::PluginLogConfiguration,
};
use ipc_channel::ipc::IpcSender;

/// Setup logging for a Plugin instance.
///
/// Given a [`PluginConfiguration`], start the configured loggers. Consumes
/// the given log channel sender.
///
/// Starts a thread-local ['LogProxy`], given a [`LoglevelFilter`] bigger
/// than [`Off`] which forwards log records to the simulator [`LogThread`].
/// Starts [`TeeFile`] loggers, given a non-empty vector of [`TeeFile`], to
/// forward log records to output files.
pub fn setup_logging(
    configuration: &PluginLogConfiguration,
    log_channel: IpcSender<LogRecord>,
) -> Result<()> {
    let mut loggers = Vec::with_capacity(1 + configuration.tee_files.len());
    loggers.push(LogProxy::boxed(
        configuration.name.as_str(),
        configuration.verbosity,
        log_channel,
    ) as Box<dyn Log>);
    loggers.extend(
        configuration
            .tee_files
            .clone()
            .into_iter()
            .map(TeeFile::create)
            .map(Box::new)
            .map(|l| l as Box<dyn Log>),
    );
    init(loggers)?;
    Ok(())
}
