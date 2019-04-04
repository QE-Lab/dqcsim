use crate::{
    common::log::{tee_file::TeeFile, LoglevelFilter},
    host::configuration::PluginProcessConfiguration,
};
use serde::{Deserialize, Serialize};

/// Configuration structure for the plugin logging system.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PluginLogConfiguration {
    /// Instance name of the plugin used to identify it in log messages.
    pub name: String,

    /// Specifies the verbosity of the messages sent to DQCsim.
    pub verbosity: LoglevelFilter,

    /// Specifies the tee files for this plugin.
    pub tee_files: Vec<TeeFile>,
}

impl PluginLogConfiguration {
    /// Returns a new PluginLogConfiguration. PluginLogConfigurations
    /// constructed using this method have no tee files setup by default.
    pub fn new(name: impl Into<String>, verbosity: LoglevelFilter) -> PluginLogConfiguration {
        PluginLogConfiguration {
            name: name.into(),
            verbosity,
            tee_files: vec![],
        }
    }
}

impl From<&PluginProcessConfiguration> for PluginLogConfiguration {
    fn from(cfg: &PluginProcessConfiguration) -> PluginLogConfiguration {
        PluginLogConfiguration {
            name: cfg.name.clone(),
            verbosity: cfg.nonfunctional.verbosity,
            tee_files: cfg.nonfunctional.tee_files.clone(),
        }
    }
}
