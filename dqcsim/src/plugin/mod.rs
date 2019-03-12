/// The Plugin configuration.
pub mod config;

mod process;

use crate::{
    plugin::{config::PluginConfig, process::PluginProcess},
    util::log::{thread::LogThread, trace},
};
use failure::{Error, Fail};
use std::{process::Command, time::Duration};

#[derive(Debug, Fail)]
pub enum PluginError {
    #[fail(display = "plugin process failed: {}", _0)]
    ProcessError(String),
}

/// The Plugin structure used in a Simulator.
pub struct Plugin {
    /// The Plugin configuration.
    config: PluginConfig,
    /// The Plugin process.
    process: PluginProcess,
}

impl Plugin {
    /// Construct a new Plugin instance.
    ///
    /// Create a Plugin instance. Starts a PluginProcess.
    pub fn new(
        config: PluginConfig,
        logger: &LogThread,
        ipc_connect_timeout: Option<Duration>,
    ) -> Result<Plugin, Error> {
        // Create the PluginProcess.
        let process = PluginProcess::new(Command::new("target/debug/dqcsim-plugin"))
            .connect(logger.get_sender().unwrap(), ipc_connect_timeout)?;
        Ok(Plugin { config, process })
    }

    /// Initialize the plugin.
    pub fn init(&mut self) -> Result<(), ()> {
        trace!("Init plugin {}", self.config.name);
        self.process.init();
        Ok(())
    }
}
