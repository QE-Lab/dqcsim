/// The Plugin configuration.
pub mod config;

mod process;
mod thread;

use crate::{
    plugin::{config::PluginConfig, thread::PluginThread},
    protocol::message,
    util::log::LogThread,
};
use failure::{Error, Fail};
use log::trace;
use std::time::Duration;

#[derive(Debug, Fail)]
pub enum PluginError {
    #[fail(display = "plugin thread failed: {}", _0)]
    ThreadError(String),
    #[fail(display = "plugin process failed: {}", _0)]
    ProcessError(String),
}

/// The Plugin structure used in a Simulator.
pub struct Plugin {
    /// The Plugin configuration.
    config: PluginConfig,
    /// The Plugin thread.
    thread: PluginThread,
}

impl Plugin {
    /// Construct a new Plugin instance.
    ///
    /// Create a Plugin instance. Starts a PluginThread controlling a
    /// PluginProcess.
    pub fn new(
        config: PluginConfig,
        logger: &LogThread,
        ipc_connect_timeout: Option<Duration>,
    ) -> Result<Plugin, Error> {
        // Create the plugin thread.
        let thread = PluginThread::new(&config, logger, ipc_connect_timeout)?;
        Ok(Plugin { config, thread })
    }

    /// Initialize the plugin.
    ///
    /// This starts the plugin thread, and initializes the control channel.
    pub fn init(&self) -> Result<(), ()> {
        trace!("Init plugin {}", self.config.name);
        self.thread.controller.send("Start".to_string()).unwrap();
        Ok(())
    }
}
