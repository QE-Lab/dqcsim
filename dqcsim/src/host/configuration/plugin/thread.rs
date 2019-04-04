use crate::{
    common::{
        error::{inv_op, Result},
        log::LoglevelFilter,
        types::{ArbCmd, PluginType},
    },
    host::{
        configuration::{PluginConfiguration, PluginLogConfiguration},
        plugin::{thread::PluginThread, Plugin},
        reproduction::{PluginReproduction, ReproductionPathStyle},
    },
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

impl Into<Box<dyn PluginConfiguration>> for PluginThreadConfiguration {
    fn into(self) -> Box<dyn PluginConfiguration> {
        Box::new(self) as Box<dyn PluginConfiguration>
    }
}

impl PluginConfiguration for PluginThreadConfiguration {
    fn instantiate(self: Box<Self>) -> Box<dyn Plugin> {
        Box::new(PluginThread::new(*self))
    }

    fn get_log_configuration(&self) -> PluginLogConfiguration {
        self.log_configuration.clone()
    }

    fn get_type(&self) -> PluginType {
        self.definition.get_type()
    }

    fn get_reproduction(&self, _: &ReproductionPathStyle) -> Result<PluginReproduction> {
        inv_op("It's not possible to build a plugin reproduction for PluginThreads")
    }

    fn limit_verbosity(&mut self, max_verbosity: LoglevelFilter) {
        if self.log_configuration.verbosity > max_verbosity {
            self.log_configuration.verbosity = max_verbosity;
        }
    }

    fn set_default_name(&mut self, default_name: String) {
        if self.log_configuration.name.is_empty() {
            self.log_configuration.name = default_name;
        }
    }
}
