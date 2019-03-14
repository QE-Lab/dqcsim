pub mod process;

use crate::{
    log::{debug, thread::LogThread, trace, LoglevelFilter},
    plugin::process::PluginProcess,
};
use failure::{bail, Error};
use std::{path::Path, process::Command, str::FromStr, string::ParseError};

#[derive(Debug, Copy, Clone)]
pub enum PluginType {
    Backend,
    Frontend,
    Operator,
}

#[derive(Debug)]
pub struct PluginConfig {
    pub name: String,
    pub loglevel: Option<LoglevelFilter>,
}

impl FromStr for PluginConfig {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<PluginConfig, ParseError> {
        Ok(PluginConfig {
            name: s.to_owned(),
            loglevel: Some(LoglevelFilter::Trace),
        })
    }
}

/// The Plugin structure used in a Simulator.
#[derive(Debug)]
pub struct Plugin {
    /// The Plugin configuration.
    config: PluginConfig,
    /// The Plugin process.
    process: PluginProcess,
    /// Command
    command: Command,
}

impl Plugin {
    /// Construct a new Plugin instance.
    ///
    /// Create a Plugin instance. Starts a PluginProcess.
    pub fn new(config: PluginConfig, logger: &LogThread) -> Result<Plugin, Error> {
        debug!("Constructing Plugin: {}", &config.name);

        let target = Path::new("target/debug/dqcsim-plugin");

        if !target.exists() || !target.is_file() {
            bail!("Plugin ({:?}) not found", target)
        }

        let mut command = Command::new(target);
        let process = PluginProcess::new(
            command.arg(&config.name),
            logger.get_sender().expect("Log thread unavailable"),
        )?;

        Ok(Plugin {
            command,
            config,
            process,
        })
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
}

impl Drop for Plugin {
    fn drop(&mut self) {
        trace!("Dropping Plugin: {}", self.config.name);
    }
}
