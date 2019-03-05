use crate::{
    plugin::{process::PluginProcess, PluginConfig, PluginError},
    protocol::message,
    util::log::{set_thread_logger, LogProxy, LogThread, Record},
};
use crossbeam_channel::{Receiver, Sender};
use failure::Error;
use ipc_channel::{
    ipc::{IpcOneShotServer, IpcReceiver},
    router::ROUTER,
};
use log::{error, info, trace};
use std::{
    process::Command,
    sync::{Arc, Condvar, Mutex},
    thread::{Builder, JoinHandle},
    time::Duration,
};

/// Thread controlling a PluginProcess.
///
///
pub struct PluginThread {
    /// The sender part of the control channel.
    ///
    /// TODO: provide methods for common control
    pub controller: crossbeam_channel::Sender<String>,
    /// The thread handler.
    handler: Option<JoinHandle<Result<(), Error>>>,
}

impl PluginThread {
    pub fn new(
        config: &PluginConfig,
        logger: &LogThread,
        ipc_connect_timeout: Option<Duration>,
    ) -> Result<PluginThread, Error> {
        // Create a channel to control the plugin thread.
        let (controller, rx): (Sender<String>, Receiver<String>) = crossbeam_channel::unbounded();

        let loglevel = config.loglevel;
        let sender = logger.get_sender().ok_or(PluginError::ThreadError(
            "Unable to get sender side of log channel".to_string(),
        ))?;

        // Spawn plugin thread.
        let handler = Builder::new()
            .name(config.name.to_string())
            .spawn(move || {
                // Setup thread local log system.
                set_thread_logger(LogProxy::boxed(sender.clone(), loglevel));
                trace!("{:?}", std::thread::current().id());

                // Create PluginProcess instance.
                let mut process = PluginProcess::new(Command::new("target/debug/dqcsim-plugin"))
                    .connect(sender)?;

                loop {
                    match rx.recv() {
                        Ok(ref msg) => match &msg[..] {
                            "Start" => {}
                            "Stop" => {
                                break;
                            }
                            _ => {}
                        },
                        Err(ref x) => {
                            log::error!("{:?}", x);
                            break;
                        }
                    }
                }
                // TODO: pipestream reader which dumps lines as they come in.
                // https://gist.github.com/ArtemGr/db40ae04b431a95f2b78
                info!("Plugin thread stopping.");
                Ok(())
            })?;
        Ok(PluginThread {
            controller,
            handler: Some(handler),
        })
    }
}

impl Drop for PluginThread {
    fn drop(&mut self) {
        // Notify the plugin thread about drop.
        self.controller.send("Stop".to_string()).unwrap();

        // Wait for the plugin thread to drop.
        self.handler
            .take()
            .expect("Plugin failed.")
            .join()
            .expect("Plugin failed.")
            .expect("Plugin thread failed.");
    }
}
