use crate::{
    common::{
        error::{inv_op, Result},
        log::LoglevelFilter,
        types::{ArbCmd, PluginType},
    },
    host::{
        configuration::{PluginConfiguration, PluginLogConfiguration, ReproductionPathStyle},
        plugin::{
            thread::{PluginThread, PluginThreadClosure},
            Plugin,
        },
        reproduction::PluginReproduction,
    },
    plugin::definition::PluginDefinition,
};
use std::fmt;

/// Represents the implementation of a plugin thread's functionality, in the
/// form of one or more closures.
#[allow(clippy::large_enum_variant)]
pub enum PluginThreadImplementation {
    /// The metadata and closures in the PluginDefinition define the behavior
    /// of the plugin.
    Definition(PluginDefinition),

    /// The plugin behavior is fully customized through a closure, taking the
    /// host address as its sole argument. The closure is called from within a
    /// worker thread. This is supposed to be equivalent to the main() function
    /// of a plugin process.
    Closure(PluginThreadClosure, PluginType),
}

impl fmt::Debug for PluginThreadImplementation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluginThreadImplementation::Definition(def) => def.fmt(fmt),
            PluginThreadImplementation::Closure(_, typ) => {
                fmt.debug_tuple("Closure").field(typ).finish()
            }
        }
    }
}

/// Represents the complete configuration for a plugin running in a local
/// thread.
#[derive(Debug)]
pub struct PluginThreadConfiguration {
    /// Implementation of the plugin thread's functionality, in the form of one
    /// or more closures.
    pub implementation: PluginThreadImplementation,

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
            implementation: PluginThreadImplementation::Definition(definition),
            init_cmds: vec![],
            log_configuration,
        }
    }

    /// Creates a new plugin through a custom closure.
    ///
    /// The default values are inserted for the configuration options.
    pub fn new_raw(
        closure: PluginThreadClosure,
        plugin_type: PluginType,
        log_configuration: PluginLogConfiguration,
    ) -> PluginThreadConfiguration {
        PluginThreadConfiguration {
            implementation: PluginThreadImplementation::Closure(closure, plugin_type),
            init_cmds: vec![],
            log_configuration,
        }
    }

    /// Adds an init cmd to the list, builder style.
    pub fn with_init_cmd(mut self, cmd: impl Into<ArbCmd>) -> PluginThreadConfiguration {
        self.init_cmds.push(cmd.into());
        self
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
        match &self.implementation {
            PluginThreadImplementation::Definition(def) => def.get_type(),
            PluginThreadImplementation::Closure(_, typ) => *typ,
        }
    }

    fn get_reproduction(&self, _: ReproductionPathStyle) -> Result<PluginReproduction> {
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
