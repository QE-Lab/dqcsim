use crate::{
    common::{error::Result, log::LoglevelFilter, types::PluginType},
    host::{
        configuration::{plugin::log::PluginLogConfiguration, ReproductionPathStyle},
        plugin::Plugin,
        reproduction::PluginReproduction,
    },
};
use std::fmt::Debug;

pub mod log;
pub mod process;
pub mod thread;

/// Trait for types of configurations. The only thing that needs to be
/// implemented is a way to instantiate a Plugin from it, and to return the
/// log configuration of the Plugin.
pub trait PluginConfiguration: Debug {
    /// Instantiates the plugin.
    fn instantiate(self: Box<Self>) -> Box<dyn Plugin>;

    /// Returns the log configuratin of the plugin. Note that this returns a
    /// copy!
    fn get_log_configuration(&self) -> PluginLogConfiguration;

    /// Returns the plugin type.
    fn get_type(&self) -> PluginType;

    /// Returns the PluginReproduction when possible. Otherwise return an
    /// error.
    fn get_reproduction(&self, path_style: ReproductionPathStyle) -> Result<PluginReproduction>;

    /// Limits the verbosity of the messages reported to the simulator.
    ///
    /// Called when the simulation is initialized to limit the plugin's
    /// verbosity to what DQCsim is actually reporting to the user. This
    /// prevents unnecessarily verbose messages from passing over the
    /// communication channels.
    fn limit_verbosity(&mut self, max_verbosity: LoglevelFilter);

    /// Sets the default name for this plugin.
    ///
    /// Called when the simulation is initialized. If the plugin did not
    /// already have an explicit name assigned to it, this value can be used.
    fn set_default_name(&mut self, default_name: String);
}

impl dyn PluginConfiguration {
    pub fn get_name(&self) -> String {
        self.get_log_configuration().name
    }
}
