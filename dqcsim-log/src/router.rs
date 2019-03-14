use crate::{init, proxy::LogProxy, trace, LevelFilter, Record};
use crossbeam_channel::Sender;
use ipc_channel::ipc::IpcReceiver;

/// Route an IpcReceiver to a crossbeam Sender.
pub fn route(receiver: IpcReceiver<Record>, sender: Sender<Record>) {
    std::thread::spawn(move || {
        init(LogProxy::boxed(sender.clone()), LevelFilter::Trace)
            .expect("Log channel forwarding failed");
        loop {
            match receiver.recv() {
                Ok(record) => sender.send(record).expect("Log channel forwarding failed"),
                Err(_) => break,
            }
        }
        trace!("Log channel forwarding stopped.");
    });
}
