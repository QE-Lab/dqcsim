pub mod process {
    use log::{info, trace};
    use std::process::{Child, Command, Stdio};
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::thread::{spawn, Builder, JoinHandle};

    pub struct Process {
        channel: (Sender<u32>, Receiver<u32>),
        handler: JoinHandle<()>,
        // child: Child,
    }

    impl Process {
        pub fn new(name: &str) -> Process {
            let channel = channel();
            let builder = Builder::new().name(name.to_owned());
            let tx = channel.0.clone();
            let handler = builder
                .spawn(move || {
                    tx.send(10).unwrap();
                })
                .unwrap();
            let rx = &channel.1;
            trace!("{}", rx.recv().unwrap());
            Process { channel, handler }
        }

        pub fn dump(&self) {
            trace!(target: self.handler.thread().name().unwrap_or(""),
                "{} running in thread: {:?}",
                self.handler.thread().name().unwrap_or(""),
                self.handler.thread().id()
            );
        }
    }
}

pub mod plugin {
    use crate::process;
    use log::trace;
    use std::{str::FromStr, string::ParseError};

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
        process: process::Process,
    }

    impl Plugin {
        pub fn init(&self) {
            trace!("Init plugin {}", self.config.name);
            self.process.dump();
        }
    }

    impl From<PluginConfig> for Plugin {
        fn from(config: PluginConfig) -> Plugin {
            Plugin {
                process: process::Process::new(&config.name),
                config,
            }
        }
    }
}
