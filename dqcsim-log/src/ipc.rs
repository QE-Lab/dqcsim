//! Utility function to spawn a log proxy connecting to a log thread via an IPC channel.

use crate::{init, proxy::LogProxy, trace, LevelFilter, Record};
use failure::Error;
use ipc_channel::ipc;

pub fn connect(server: impl Into<String>, level: LevelFilter) -> Result<(), Error> {
    // Connect to the server.
    let connect = ipc::IpcSender::connect(server.into())?;

    // Create log channel
    let (tx, rx): (ipc::IpcSender<Record>, _) = ipc::channel()?;
    // Send receiver to host.
    connect.send(rx)?;

    // Initialize thread local logger.
    init(LogProxy::boxed(tx), level)?;

    trace!("Connected to log channel via ipc");
    Ok(())
}
