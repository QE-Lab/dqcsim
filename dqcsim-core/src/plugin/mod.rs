use ipc_channel::ipc::{channel, IpcReceiver, IpcSender};
use log::{info, trace, warn};
use std::{
    process::{Child, Command, Stdio},
    thread::{Builder, JoinHandle},
};

pub mod config;

use crate::plugin::config::PluginConfig;

pub struct Plugin {
    config: PluginConfig,
    channel: (IpcSender<u32>, IpcReceiver<u32>),
    // handler: JoinHandle<()>,
    child: Child,
}

impl Plugin {
    pub fn init(&self) {
        trace!("Init plugin {}", self.config.name);
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        warn!("Shutting down...");
        let result = self.child.wait();
        trace!("{:?}", result);
        warn!("Down...");
    }
}

impl From<PluginConfig> for Plugin {
    fn from(config: PluginConfig) -> Plugin {
        let channel = channel().unwrap();
        let builder = Builder::new().name(config.name.to_owned());
        let tx = channel.0.clone();
        let handler = builder
            .spawn(move || {
                tx.send(10).unwrap();
                std::thread::sleep(std::time::Duration::from_micros(1));
                warn!("warning from thread");
            })
            .unwrap();
        let rx = &channel.1;
        trace!("{}", rx.recv().unwrap());
        handler.join();
        let mut child = Command::new("sleep")
            .arg("5")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to start echo process");
        trace!("{}", &child.id());
        Plugin {
            config,
            channel,
            // handler,
            child,
        }
    }
}
