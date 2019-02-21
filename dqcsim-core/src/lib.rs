pub mod ipc;
pub mod plugin;
pub mod simulator;

// pub mod process {
//     use log::trace;
//     use std::process::{Child, Command, Stdio};
//     use std::sync::mpsc::{channel, Receiver, Sender};
//     use std::thread::{Builder, JoinHandle};
//
//     pub struct Process {
//         channel: (Sender<u32>, Receiver<u32>),
//         handler: JoinHandle<()>,
//         // child: Child,
//     }
//
//     impl Process {
//         pub fn new(name: &str) -> Process {
//             let channel = channel();
//             let builder = Builder::new().name(name.to_owned());
//             let tx = channel.0.clone();
//             let handler = builder
//                 .spawn(move || {
//                     tx.send(10).unwrap();
//                 })
//                 .unwrap();
//             let rx = &channel.1;
//             trace!("{}", rx.recv().unwrap());
//             Process { channel, handler }
//         }
//
//         pub fn dump(&self) {
//             trace!(target: self.handler.thread().name().unwrap_or(""),
//                 "{} running in thread: {:?}",
//                 self.handler.thread().name().unwrap_or(""),
//                 self.handler.thread().id()
//             );
//         }
//     }
// }
