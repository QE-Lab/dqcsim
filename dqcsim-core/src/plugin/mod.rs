use crate::{ipc::message::PluginControl, plugin::config::PluginConfig};
use crossbeam_channel::unbounded;
use dqcsim_log::LogThread;
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use log::{error, info, trace, warn};
use std::{
    error::Error,
    process::{Child, Command, Stdio},
    thread::{Builder, JoinHandle},
};

/// The Plugin configuration.
pub mod config;

/// The Plugin structure used in a Simulator
pub struct Plugin {
    /// The configuration.
    config: PluginConfig,
    /// The thread handler.
    handler: Option<JoinHandle<()>>,
    /// The sender part of the control channel.
    controller: crossbeam_channel::Sender<PluginControl>,
}

/// The plugin thread control function.

impl Plugin {
    pub fn new(config: PluginConfig, logger: &LogThread) -> Plugin {
        // Create a channel to control the plugin thread.
        let (controller, rx) = crossbeam_channel::unbounded();

        // Spawn thread for the plugin.
        let name = config.name.clone();
        let handler = Builder::new()
            .name(config.name.to_owned())
            .spawn(move || {
                info!("[{}] Plugin thread started.", name);
                loop {
                    match rx.recv() {
                        Ok(msg) => match msg {
                            PluginControl::Start => {
                                trace!("start");
                            }
                            PluginControl::Abort => {
                                trace!("abort");
                                break;
                            }
                        },
                        Err(x) => {
                            error!("{:?}", x.description());
                            break;
                        }
                    }
                }
                info!("[{}] Plugin thread stopping.", name);
            })
            .ok();
        // let builder = Builder::new().name(config.name.to_owned());
        // let handler = builder
        //     .spawn(move || {
        //         warn!("Plugin thread started.");
        //         for msg in rx.into_iter() {
        //             match msg {
        //                 PluginControl::Start => {
        //                     trace!("start");
        //                 }
        //                 PluginControl::Abort => {
        //                     trace!("abort");
        //                     break;
        //                 }
        //             }
        //         }
        //         info!("Plugin thread stopping.");
        //     })
        //     .expect("Spawning plugin thread failed.");

        // tx.send(PluginControl::Start).unwrap();
        // tx.send(PluginControl::Abort).unwrap();
        // // drop(tx);
        // handler.join().expect("Plugin thread failed.");

        // // Setup control channel
        // let (server, server_name): (IpcOneShotServer<IpcReceiver<u32>>, String) =
        //     IpcOneShotServer::new().unwrap();
        // trace!("Server for {}: {}", &config.name, server_name);
        //
        // let mut child = Command::new("cargo")
        //     .arg("run")
        //     .arg("-p")
        //     .arg("dqcsim-plugin")
        //     .arg("--")
        //     .arg(server_name)
        //     .stderr(Stdio::null())
        //     .stdout(Stdio::null())
        //     .spawn()
        //     .expect("Failed to start echo process");
        // trace!(
        //     "Started child process for {}: {}",
        //     &config.name,
        //     &child.id()
        // );
        //
        // // Wait for child process to connect and send the receiver.
        // let (_, receiver): (_, IpcReceiver<u32>) = server.accept().unwrap();
        //
        // // Get some messages over the new channel.
        // trace!("{}", receiver.recv().unwrap());
        // trace!("{}", receiver.recv().unwrap());

        // let (tx, receiver) = ipc::channel().unwrap();
        //
        // let builder = Builder::new().name(config.name.to_owned());
        // let handler = builder
        //     .spawn(move || {
        //         tx.send(10).unwrap();
        //         std::thread::sleep(std::time::Duration::from_micros(1));
        //         warn!("warning from thread");
        //     })
        //     .unwrap();
        // trace!("{}", receiver.recv().unwrap());
        // handler.join();

        // let (_, data): (_, u32) = server.accept().unwrap();
        // info!("data from child: {}", data);
        Plugin {
            config,
            handler,
            controller,
        }
    }
    /// Initialize the plugin.
    /// This starts the plugin thread, and initializes the control channel.
    pub fn init(&self) -> Result<(), ()> {
        trace!("Init plugin {}", self.config.name);
        self.controller.send(PluginControl::Start).unwrap();
        Ok(())
    }

    pub fn wait(mut self) {
        info!("Waiting for plugin to stop");
        self.controller.send(PluginControl::Abort).unwrap();
        self.handler.unwrap().join().expect("Plugin thread failed.");
        info!("Plugin stopped");
        self.handler = None;
    }
}

// impl Drop for Plugin {
//     fn drop(&mut self) {
//         warn!("Shutting down...");
//         // let result = self.child.wait();
//         // trace!("{:?}", result);
//         warn!("Down...");
//     }
// }
//
// impl From<PluginConfig> for Plugin {
//     fn from(config: PluginConfig) -> Plugin {}
// }
