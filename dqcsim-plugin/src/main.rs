use ipc_channel::ipc::{channel, IpcOneShotServer, IpcReceiver, IpcSender};
use log::{info, trace, warn};
use std::{
    env,
    process::{Child, Command, Stdio},
    thread::{Builder, JoinHandle},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = &args[1];
    trace!("{}", &server);

    let connect = IpcSender::connect(server.to_owned()).unwrap();

    // Create channel
    let (tx, rx) = channel().unwrap();
    // Send receiver to host.
    connect.send(rx).unwrap();

    // Send some test messages over the established ipc connection
    tx.send(1234).unwrap();
    tx.send(12345).unwrap();
}
