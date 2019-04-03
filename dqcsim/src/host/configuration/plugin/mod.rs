use crate::{
    common::{error::Result, types::PluginType},
    host::{
        configuration::plugin::log::PluginLogConfiguration,
        plugin::Plugin,
        reproduction::{PluginReproduction, ReproductionPathStyle},
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

    /// Returns the log configuratin of the plugin.
    fn log_configuration(&self) -> PluginLogConfiguration;

    /// Returns the plugin type of the plugin.
    fn get_type(&self) -> PluginType;

    /// Sets the typ of the plugin.
    fn set_type(&mut self, typ: PluginType);

    /// Sets the name of the plugin.
    fn set_name(&mut self, name: String);

    /// Returns the PluginReproduction when possible. Otherwise return an
    /// error.
    fn get_reproduction(&self, path_style: &ReproductionPathStyle) -> Result<PluginReproduction>;
}

impl PluginConfiguration {
    pub fn name(&self) -> String {
        self.log_configuration().name
    }
}
