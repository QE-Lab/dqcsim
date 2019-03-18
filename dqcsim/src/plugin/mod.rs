pub mod process;

use crate::{
    configuration::{ArbCmd, ArbData, PluginConfiguration},
    debug,
    log::thread::LogThread,
    plugin::process::PluginProcess,
    trace,
};
use failure::{bail, Error};
use std::{path::Path, process::Command};

/// The Plugin structure used in a Simulator.
#[derive(Debug)]
pub struct Plugin {
    /// The Plugin configuration.
    configuration: PluginConfiguration,
    /// The Plugin process.
    process: PluginProcess,
    /// Command
    command: Command,
}

impl Plugin {
    /// Construct a new Plugin instance.
    ///
    /// Create a Plugin instance. Starts a PluginProcess.
    pub fn new(configuration: PluginConfiguration, logger: &LogThread) -> Result<Plugin, Error> {
        debug!("Constructing Plugin: {}", &configuration.name);

        let target = Path::new("target/debug/dqcsim-plugin");

        if !target.exists() || !target.is_file() {
            bail!("Plugin ({:?}) not found", target)
        }

        let mut command = Command::new(target);
        let process = PluginProcess::new(
            command.arg(&configuration.name),
            logger.get_sender().expect("Log thread unavailable"),
        )?;

        Ok(Plugin {
            command,
            configuration,
            process,
        })
    }

    /// Returns the name of the plugin.
    pub fn name(&self) -> &str {
        &self.configuration.name
    }

    /// Init
    pub fn init<'a>(
        &self,
        downstream: Option<String>,
        upstream: &mut impl Iterator<Item = &'a Plugin>,
    ) -> Result<(), Error> {
        self.process.init(downstream, upstream)?;
        Ok(())
    }

    /// Abort
    pub fn abort(&mut self, graceful: bool) {
        if let Ok(Some(exit)) = self.process.abort(graceful) {
            debug!("Plugin process already exited: {}", exit);
        }
    }

    /// Sends an `ArbCmd` message to this plugin.
    #[allow(unused)] // TODO: remove <--
    pub fn arb(&mut self, cmd: impl Into<ArbCmd>) -> Result<ArbData, Error> {
        // TODO
        bail!("Not yet implemented")
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        trace!("Dropping Plugin: {}", self.configuration.name);
    }
}
