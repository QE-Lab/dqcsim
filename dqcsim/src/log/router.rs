use crate::{
    log::{init, proxy::LogProxy, LoglevelFilter, Record},
    trace,
};
use crossbeam_channel::Sender;
use ipc_channel::ipc::IpcReceiver;

/// Route an IpcReceiver to a crossbeam Sender.
pub fn route(name: impl Into<String>, receiver: IpcReceiver<Record>, sender: Sender<Record>) {
    let name = name.into();
    std::thread::spawn(move || {
        init(LogProxy::boxed(name, sender.clone()), LoglevelFilter::Trace)
            .expect("Log channel forwarding failed");
        while let Ok(record) = receiver.recv() {
            sender.send(record).expect("Log channel forwarding failed");
        }
        trace!("Log channel forwarding stopped.");
    });
}
