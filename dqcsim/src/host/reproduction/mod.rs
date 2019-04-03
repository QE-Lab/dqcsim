//! Simulation run reproduction functionality.

use crate::{
    common::{
        error::{err, Result},
        types::PluginType,
    },
    host::configuration::*,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod host_call;
pub use host_call::HostCall;

mod path_style;
pub use path_style::ReproductionPathStyle;

/// The contents of a plugin configuration in a reproduction file.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginReproduction {
    /// Name of the plugin, used to refer to the plugin by the log system.
    pub name: String,

    /// The executable filename of the plugin.
    pub executable: PathBuf,

    /// If specified, the executable is expected to be an interpreter, which is
    /// to execute the specified script file. If not specified, the executable
    /// is a native plugin.
    pub script: Option<PathBuf>,

    /// The functional configuration of the plugin, i.e. the options
    /// configuring how the plugin behaves (besides the specification).
    #[serde(flatten)]
    pub functional: PluginProcessFunctionalConfiguration,
}

/// The contents of a reproduction file.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Reproduction {
    /// The random seed for the simulation.
    pub seed: u64,

    /// The plugin configurations, from front to back.
    pub plugins: Vec<PluginReproduction>,

    /// The sequence of host calls to make.
    ///
    /// Note that `wait()` is not represented in the `HostCall` enumeration.
    /// `wait()` calls should instead be inserted automatically as late as
    /// possible, that is:
    ///
    ///  - when `HostCall::Start` is encountered while the accelerator was
    ///    already started;
    ///  - before DQCsim terminates, if the accelerator is still running.
    pub host_calls: Vec<HostCall>,

    /// The host on which the run was performed, if available.
    ///
    /// This parameter is not used by DQCsim when it runs in reproduction mode.
    /// It is only there for helping the user trace the reproduction file back
    /// to its source.
    pub hostname: String,

    /// The user that performed the run.
    ///
    /// This parameter is not used by DQCsim when it runs in reproduction mode.
    /// It is only there for helping the user trace the reproduction file back
    /// to its source.
    pub username: String,

    /// The working directory that the run was performed in.
    ///
    /// This parameter is not used by DQCsim when it runs in reproduction mode.
    /// It is only there for helping the user trace the reproduction file back
    /// to its source.
    pub workdir: PathBuf,
}

impl Reproduction {
    /// Constructs a reproduction structure from a completed run.
    pub fn from_run<'a, T>(
        config: &SimulatorConfiguration,
        host_calls: T,
        path_style: ReproductionPathStyle,
    ) -> Result<Reproduction>
    where
        T: IntoIterator<Item = &'a HostCall>,
    {
        Ok(Reproduction {
            seed: config.seed.value,
            host_calls: host_calls.into_iter().cloned().collect(),
            plugins: config
                .plugins
                .iter()
                .map(|x| x.get_reproduction(&path_style))
                .collect::<Result<Vec<PluginReproduction>>>()?,
            hostname: whoami::hostname(),
            username: whoami::username(),
            workdir: std::env::current_dir()?,
        })
    }

    /// Turns this reproduction structure into a configuration and a list of
    /// host calls for reproduction.
    ///
    /// If exact is set, the random seed is taken from the reproduction
    /// structure. Otherwise, it is regenerated.
    pub fn to_run(
        &self,
        config: &mut SimulatorConfiguration,
        exact: bool,
    ) -> Result<Vec<HostCall>> {
        // If this is an exact reproduction, set the seed.
        if exact {
            config.seed.value = self.seed;
        }

        // Construct the plugin configurations. The nonfunctional config is set
        // to the default value. Note that we pretend that every plugin is an
        // operator here; we change this later.
        config.plugins = self
            .plugins
            .iter()
            .map(|x| PluginProcessConfiguration {
                name: x.name.clone(),
                specification: PluginProcessSpecification::new(
                    &x.executable,
                    x.script.clone(),
                    PluginType::Operator,
                ),
                functional: x.functional.clone(),
                nonfunctional: PluginProcessNonfunctionalConfiguration::default(),
            })
            .map(|plugin| Box::new(plugin) as Box<dyn PluginConfiguration>)
            .collect::<Vec<Box<dyn PluginConfiguration>>>();

        // Set the plugin types and make sure we have at least a frontend
        // and a backend.
        let plugin_count = config.plugins.len();
        if plugin_count < 2 {
            err("reproduction file corrupted: less than two plugins specified")?;
        }
        config.plugins[0].set_type(PluginType::Frontend);
        config.plugins[plugin_count - 1].set_type(PluginType::Backend);

        Ok(self.host_calls.clone())
    }

    /// Constructs a reproduction structure from a file.
    pub fn from_file(file: &Path) -> Result<Reproduction> {
        Ok(serde_yaml::from_reader(&mut std::fs::File::open(file)?)?)
    }

    /// Writes a reproduction structure to a file.
    pub fn to_file(&self, file: &Path) -> Result<()> {
        serde_yaml::to_writer(&mut std::fs::File::create(file)?, self)?;
        Ok(())
    }
}
