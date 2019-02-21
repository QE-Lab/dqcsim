use log::{trace, warn};
use std::{
    process::{Child, Command, Stdio},
    str::FromStr,
    string::ParseError,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{Builder, JoinHandle},
};

#[derive(Debug)]
pub struct PluginConfig {
    name: String,
}

impl FromStr for PluginConfig {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<PluginConfig, ParseError> {
        Ok(PluginConfig { name: s.to_owned() })
    }
}

pub struct Plugin {
    config: PluginConfig,
    channel: (Sender<u32>, Receiver<u32>),
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
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

impl From<PluginConfig> for Plugin {
    fn from(config: PluginConfig) -> Plugin {
        let channel = channel();
        let builder = Builder::new().name(config.name.to_owned());
        let tx = channel.0.clone();
        let handler = builder
            .spawn(move || {
                tx.send(10).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
                warn!("warning from thread");
            })
            .unwrap();
        let rx = &channel.1;
        trace!("{}", rx.recv().unwrap());
        handler.join();
        let mut child = Command::new("echo")
            .arg("Hello world!")
            .spawn()
            .expect("Failed to start echo process");
        child.wait();
        Plugin {
            config,
            channel,
            // handler,
            child,
        }
    }
}
