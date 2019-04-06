//! Simulation run reproduction functionality.

use crate::{
    common::{
        error::{err, inv_arg, oe_inv_arg, Result},
        log::{tee_file::TeeFile, LoglevelFilter},
        types::PluginType,
    },
    host::configuration::*,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod host_call;
pub use host_call::HostCall;

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

/// Represents a nonfunctional configuration modification for a previously
/// defined plugin.
///
/// This allows the nonfunctional configuration of the plugin to be modified.
/// This is particularly important when DQCsim is running in
/// --reproduce-exactly mode; it is of marginal use to reproduce a run exactly
/// if you're not looking to get additional information from it by changing
/// loglevels and such.
#[derive(Debug)]
pub struct PluginModification {
    /// Name of the referenced plugin.
    pub name: String,

    /// Specifies the verbosity of the messages sent to DQCsim. If this is
    /// `None`, the value of DQCsim's `--plugin_level` option should be used.
    pub verbosity: Option<LoglevelFilter>,

    /// Specifies the tee files for this plugin.
    pub tee_files: Vec<TeeFile>,

    /// Specifies how the stdout stream of the plugin should be connected.
    /// `None` implies default.
    pub stdout_mode: Option<StreamCaptureMode>,

    /// Specifies how the stderr stream of the plugin should be connected.
    /// `None` implies default.
    pub stderr_mode: Option<StreamCaptureMode>,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub accept_timeout: Option<Timeout>,

    /// Specifies the timeout for connecting to the plugin after it has been
    /// spawned.
    pub shutdown_timeout: Option<Timeout>,
}

impl PluginModification {
    /// Applies this plugin modification to a plugin definition vector.
    ///
    /// An error is returned if the referenced plugin cannot be found in the
    /// vector, otherwise `Ok(())` is returned.
    pub fn apply(self, to: &mut Vec<PluginProcessConfiguration>) -> Result<()> {
        for plugin_config in &mut to.iter_mut() {
            if plugin_config.name == self.name {
                if let Some(verbosity) = self.verbosity {
                    plugin_config.nonfunctional.verbosity = verbosity;
                }
                plugin_config
                    .nonfunctional
                    .tee_files
                    .extend(self.tee_files.iter().cloned());
                if let Some(stdout_mode) = &self.stdout_mode {
                    plugin_config.nonfunctional.stdout_mode = stdout_mode.clone();
                }
                if let Some(stderr_mode) = &self.stderr_mode {
                    plugin_config.nonfunctional.stderr_mode = stderr_mode.clone();
                }
                if let Some(accept_timeout) = &self.accept_timeout {
                    plugin_config.nonfunctional.accept_timeout = *accept_timeout;
                }
                if let Some(shutdown_timeout) = &self.shutdown_timeout {
                    plugin_config.nonfunctional.shutdown_timeout = *shutdown_timeout;
                }
                return Ok(());
            }
        }
        inv_arg(format!(
            "There is no plugin named {}. The available plugins are {}.",
            self.name,
            enum_variants::friendly_enumerate(to.iter().map(|x| &x.name[..]), Some("or"))
        ))
    }
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
    /// Constructs a reproduction structure for tracking a simulation.
    pub fn new_logger(config: &SimulatorConfiguration) -> Result<Reproduction> {
        Ok(Reproduction {
            seed: config.seed.value,
            host_calls: vec![],
            plugins: config
                .plugins
                .iter()
                .map(|x| x.get_reproduction(config.reproduction_path_style.ok_or_else(oe_inv_arg("cannot create reproduction logger for simulator configuration with reproduction explicitly disabled"))?))
                .collect::<Result<Vec<PluginReproduction>>>()?,
            hostname: whoami::hostname(),
            username: whoami::username(),
            workdir: std::env::current_dir()?,
        })
    }

    /// Records a host call to the reproduction log.
    pub fn record(&mut self, host_call: HostCall) {
        self.host_calls.push(host_call);
    }

    /// Turns this reproduction structure into a configuration and a list of
    /// host calls for reproduction.
    ///
    /// If exact is set, the random seed is taken from the reproduction
    /// structure. Otherwise, it is regenerated.
    pub fn to_run(
        &self,
        config: &mut SimulatorConfiguration,
        modifications: impl IntoIterator<Item = PluginModification>,
        exact: bool,
    ) -> Result<Vec<HostCall>> {
        // If this is an exact reproduction, set the seed.
        if exact {
            config.seed.value = self.seed;
        }

        // Construct the plugin configurations. The nonfunctional config is set
        // to the default value. Note that we pretend that every plugin is an
        // operator here; we change this later.
        let mut plugins = self
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
            .collect::<Vec<PluginProcessConfiguration>>();

        // Set the plugin types and make sure we have at least a frontend
        // and a backend.
        let plugin_count = plugins.len();
        if plugin_count < 2 {
            err("reproduction file corrupted: less than two plugins specified")?;
        }
        plugins[0].specification.typ = PluginType::Frontend;
        plugins[plugin_count - 1].specification.typ = PluginType::Backend;

        // Update the plugin nonfunctional configurations using the
        // modification list.
        for m in modifications {
            m.apply(&mut plugins)?;
        }

        // Wrap the PluginProcessConfigurations in a box for the simulator
        // configuration structure.
        config.plugins = plugins
            .into_iter()
            .map(|plugin| Box::new(plugin) as Box<dyn PluginConfiguration>)
            .collect();

        Ok(self.host_calls.clone())
    }

    /// Constructs a reproduction structure from a file.
    pub fn from_file(file: impl AsRef<Path>) -> Result<Reproduction> {
        Ok(serde_yaml::from_reader(&mut std::fs::File::open(
            file.as_ref(),
        )?)?)
    }

    /// Writes a reproduction structure to a file.
    pub fn to_file(&self, file: impl AsRef<Path>) -> Result<()> {
        serde_yaml::to_writer(&mut std::fs::File::create(file.as_ref())?, self)?;
        Ok(())
    }
}
