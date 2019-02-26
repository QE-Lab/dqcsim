use crate::{ipc::message::PluginControl, plugin::config::PluginConfig};
use dqcsim_log::{LogProxy, LogThread};
use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver};
use log::{error, info, trace};
use std::{
    error::Error,
    io::Read,
    process::{Command, Stdio},
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
        let sender = logger
            .get_sender()
            .expect("Unable to get sender side of log channel.");
        let handler = Builder::new()
            .name(config.name.to_owned())
            .spawn(move || {
                dqcsim_log::set_thread_logger(Box::new(LogProxy::new(sender, None)));
                info!(
                    "[{}] Plugin running in thread: {:?}",
                    &name,
                    std::thread::current().id()
                );
                loop {
                    match rx.recv() {
                        Ok(msg) => match msg {
                            PluginControl::Start => {
                                info!("start");
                                // Setup control channel
                                let (server, server_name): (
                                    IpcOneShotServer<IpcReceiver<String>>,
                                    String,
                                ) = IpcOneShotServer::new().unwrap();
                                trace!("Server for {}: {}", &name, server_name);

                                let mut child = Command::new("target/debug/dqcsim-plugin")
                                    .arg(server_name)
                                    .stderr(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("Failed to start echo process");

                                trace!("Started child process for {}: {}", &name, &child.id());

                                // Wait for child process to connect and send the receiver.
                                let (_, receiver): (_, IpcReceiver<String>) =
                                    server.accept().unwrap();

                                // Get a message.
                                trace!("message from client: {}", receiver.recv().unwrap());

                                // Wait for child to finish
                                trace!("child stopped: {}", child.wait().expect("child failed."));

                                // Dump stdout
                                let mut stdout = String::new();
                                child
                                    .stdout
                                    .take()
                                    .unwrap()
                                    .read_to_string(&mut stdout)
                                    .expect("stdout read failed.");
                                let mut stderr = String::new();
                                child
                                    .stderr
                                    .take()
                                    .unwrap()
                                    .read_to_string(&mut stderr)
                                    .expect("stderr read failed.");
                                trace!("{}", stdout);
                                trace!("{}", stderr);
                            }
                            PluginControl::Abort => {
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
}

impl Drop for Plugin {
    fn drop(&mut self) {
        self.controller.send(PluginControl::Abort).unwrap();
        self.handler
            .take()
            .expect("Plugin failed.")
            .join()
            .expect("Plugin failed.");
    }
}
