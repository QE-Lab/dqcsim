use crate::{
    common::types::ArbCmd, host::configuration::PluginLogConfiguration,
    plugin::definition::PluginDefinition,
};

/// Represents the complete configuration for a plugin running in a local
/// thread.
#[derive(Debug)]
pub struct PluginThreadConfiguration {
    /// The metadata and closures that define the behavior of the plugin.
    pub definition: PluginDefinition,

    /// The vector of `ArbCmd`s passed to the `initialize()` closure.
    ///
    /// This is mostly useless since you can also just move data directly into
    /// the closures prior to plugin construction. Therefore it is not part of
    /// the constructor function. It is mostly just left here for uniformity
    /// with the external process method of constructing a plugin.
    pub init_cmds: Vec<ArbCmd>,

    /// Configuration for the logging subsystem of the plugin.
    pub log_configuration: PluginLogConfiguration,
}

impl PluginThreadConfiguration {
    /// Creates a new plugin configuration.
    ///
    /// The default values are inserted for the configuration options.
    pub fn new(
        definition: PluginDefinition,
        log_configuration: PluginLogConfiguration,
    ) -> PluginThreadConfiguration {
        PluginThreadConfiguration {
            definition,
            init_cmds: vec![],
            log_configuration,
        }
    }
}
