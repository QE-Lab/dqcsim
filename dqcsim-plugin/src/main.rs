use ipc_channel::ipc::{channel, IpcSender};
use log::trace;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server = &args[1];
    trace!("{}", &server);

    println!("stdout from  {}", std::process::id());
    eprintln!("stderr from {}", std::process::id());

    let connect = IpcSender::connect(server.to_owned()).unwrap();

    // Create channel
    let (tx, rx) = channel().unwrap();
    // Send receiver to host.
    connect.send(rx).expect("Unable to send receiver to host.");

    // Send a test message over the established ipc connection
    tx.send(format!("client connected to {}", server)).unwrap();
}
