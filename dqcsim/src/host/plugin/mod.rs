pub mod process;

use crate::{
    common::{
        error::{ErrorKind, Result},
        log::Record,
        protocol::{InitializeRequest, Request, Response},
    },
    host::{
        configuration::{ArbCmd, ArbData, PluginConfiguration},
        plugin::process::PluginProcess,
    },
    trace,
};
use std::process::Command;

/// The Plugin structure used in a Simulator.
#[derive(Debug)]
pub struct Plugin {
    /// The Plugin configuration.
    configuration: PluginConfiguration,
    /// The Plugin process.
    process: Option<PluginProcess>,
    /// Command
    command: Command,
}

impl Plugin {
    /// Construct a new Plugin instance.
    ///
    /// Create a Plugin instance.
    pub fn try_from(configuration: PluginConfiguration) -> Result<Plugin> {
        trace!("Constructing Plugin: {}", &configuration.name);

        let target = &configuration.specification.executable;

        if !target.is_file() {
            Err(ErrorKind::InvalidArgument(format!(
                "Plugin ({:?}) not found",
                target
            )))?
        }

        let mut command = Command::new(target);

        // We don't check for existence as this is not neccesarily a existing path.
        // It's the responsibility of the plugin to check this.
        if configuration.specification.script.is_some() {
            command.arg(configuration.specification.script.as_ref().unwrap());
        }

        Ok(Plugin {
            configuration,
            process: None,
            command,
        })
    }

    /// Returns the name of the plugin.
    pub fn name(&self) -> &str {
        &self.configuration.name
    }

    /// Returns a reference to the plugin's command.
    #[doc(hidden)]
    pub fn command(&self) -> &Command {
        &self.command
    }

    fn process_ref(&self) -> &PluginProcess {
        self.process.as_ref().unwrap()
    }

    pub fn spawn(&mut self, log_sender: crossbeam_channel::Sender<Record>) -> Result<()> {
        let process = PluginProcess::new(
            &self.configuration,
            self.command.arg(&self.configuration.name),
            log_sender,
        )?;
        self.process = Some(process);
        Ok(())
    }

    /// Initialize the Plugin.
    ///
    /// Initializes the [`Plugin`] by sending an initialization
    /// [`Request::Init`] to the [`PluginProcess`].
    pub fn init<'a>(
        &self,
        downstream: Option<String>,
        upstream: &mut impl Iterator<Item = &'a Plugin>,
    ) -> Result<()> {
        trace!("Initialize Plugin: {}", self.configuration.name);
        self.process_ref()
            .request(Request::Init(InitializeRequest {
                downstream,
                arb_cmds: self.configuration.functional.init.clone(),
                prefix: self.configuration.name.to_owned(),
                level: self.configuration.nonfunctional.verbosity,
            }))?;
        match self.process_ref().wait_for_reply() {
            Response::Init(response) => {
                trace!("Got reponse: {:?}", response);
                if let Some(upstream_plugin) = upstream.next() {
                    trace!("Connecting to upstream plugin");
                    upstream_plugin.init(response.upstream, upstream)?;
                }
                Ok(())
            }
            _ => Err(ErrorKind::Other("bad-reply".to_string()))?,
        }
    }

    /// Sends an `ArbCmd` message to this plugin.
    #[allow(unused)] // TODO: remove <--
    pub fn arb(&mut self, cmd: impl Into<ArbCmd>) -> Result<ArbData> {
        // TODO
        Err(ErrorKind::Other("Not yet implemented".to_string()))?
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        trace!("Dropping Plugin: {}", self.configuration.name);
    }
}

#[cfg(test)]
mod tests {
    use super::Plugin;
    use crate::host::configuration::{PluginConfiguration, PluginSpecification, PluginType};
    use std::process::Command;

    #[test]
    fn with_script() {
        let current = std::env::current_exe().unwrap();

        let spec =
            PluginSpecification::new(current.clone(), Some(current.clone()), PluginType::Operator);

        let configuration = PluginConfiguration::new("test", spec);
        let plugin = Plugin::try_from(configuration);
        assert!(plugin.is_ok());

        let plugin = plugin.unwrap();
        assert_eq!(
            format!("{:?}", plugin.command()),
            format!("{:?}", Command::new(current.clone()).arg(current))
        );
    }
}
