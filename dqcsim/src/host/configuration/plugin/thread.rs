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
    plugin::{definition::PluginDefinition, state::PluginState},
    trace,
};
use std::fmt;

/// Represents the complete configuration for a plugin running in a local
/// thread.
pub struct PluginThreadConfiguration {
    /// The closure that's called from within the plugin thread. It is
    /// responsible for calling `PluginState::run()` in one way or another.
    pub closure: PluginThreadClosure,

    /// The type of plugin that the closure is expected to start.
    pub plugin_type: PluginType,

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

impl fmt::Debug for PluginThreadConfiguration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PluginThreadConfiguration")
            .field("closure", &"...".to_string())
            .field("plugin_type", &self.plugin_type)
            .field("init_cmds", &self.init_cmds)
            .field("log_configuration", &self.log_configuration)
            .finish()
    }
}

impl PluginThreadConfiguration {
    /// Creates a new plugin configuration.
    ///
    /// The default values are inserted for the configuration options.
    pub fn new(
        definition: PluginDefinition,
        log_configuration: PluginLogConfiguration,
    ) -> PluginThreadConfiguration {
        let plugin_type = definition.get_type();
        PluginThreadConfiguration::new_raw(
            Box::new(move |server| {
                PluginState::run(&definition, server).expect("Plugin thread failed");
                trace!("$");
            }),
            plugin_type,
            log_configuration,
        )
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
            closure,
            plugin_type,
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
        self.plugin_type
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
