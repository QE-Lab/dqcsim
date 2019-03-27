//! Router functionality.

use crate::{
    common::log::{init, proxy::LogProxy, LogRecord, LoglevelFilter},
    trace,
};
use crossbeam_channel::Sender;
use ipc_channel::ipc::IpcReceiver;

/// Route an IpcReceiver to a crossbeam Sender.
pub fn route(
    name: impl Into<String>,
    level: LoglevelFilter,
    receiver: IpcReceiver<LogRecord>,
    sender: Sender<LogRecord>,
) {
    let name = name.into();
    std::thread::spawn(move || {
        init(vec![LogProxy::boxed(name, level, sender.clone())])
            .expect("Log channel forwarding failed");
        while let Ok(record) = receiver.recv() {
            sender.send(record).expect("Log channel forwarding failed");
        }
        trace!("Log channel forwarding stopped.");
    });
}
