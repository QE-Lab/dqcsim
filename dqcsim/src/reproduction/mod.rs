use crate::{
    configuration::*,
    error::{err, Result},
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
    pub functional: PluginFunctionalConfiguration,
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
                .map(|x| {
                    Ok(PluginReproduction {
                        name: x.name.clone(),
                        executable: path_style.convert_path(&x.specification.executable)?,
                        script: path_style.convert_path_option(&x.specification.script)?,
                        functional: PluginFunctionalConfiguration {
                            init: x.functional.init.clone(),
                            env: x.functional.env.clone(),
                            work: path_style.convert_path(&x.functional.work)?,
                        },
                    })
                })
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
            .map(|x| {
                Ok(PluginConfiguration {
                    name: x.name.clone(),
                    specification: PluginSpecification::new(
                        &x.executable,
                        x.script.clone(),
                        PluginType::Operator,
                    ),
                    functional: x.functional.clone(),
                    nonfunctional: PluginNonfunctionalConfiguration::default(),
                })
            })
            .collect::<Result<_>>()?;

        // Set the plugin types and make sure we have at least a frontend
        // and a backend.
        let plugin_count = config.plugins.len();
        if plugin_count < 2 {
            err("reproduction file corrupted: less than two plugins specified")?;
        }
        config.plugins[0].specification.typ = PluginType::Frontend;
        config.plugins[plugin_count - 1].specification.typ = PluginType::Frontend;

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
