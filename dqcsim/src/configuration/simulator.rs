use super::plugin::PluginConfiguration;
use super::seed::Seed;
use dqcsim_log::{LoglevelFilter, tee_file::TeeFile};
use serde::{Deserialize, Serialize};

/// The complete configuration for a DQCsim run.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimulatorConfiguration {
    /// The random seed for the simulation.
    pub seed: Seed,

    /// The verbosity for logging messages to stderr.
    pub stderr_level: LoglevelFilter,

    /// Logs messages to the specified file in addition to stderr. level sets
    /// the minimum importance for a message to be logged to this file.
    pub tee_files: Vec<TeeFile>,

    /// The verbosity for DQCsim itself.
    pub dqcsim_level: LoglevelFilter,

    /// The plugin configurations, from front to back.
    pub plugins: Vec<PluginConfiguration>,
}

impl SimulatorConfiguration {
    /// Optimizes the source verbosity levels, such that they are no more
    /// verbose than the most verbose sink.
    pub fn optimize_loglevels(&mut self) {
        // Find the verbosity of the most verbose sink.
        let mut max_dqcsim_verbosity = self.stderr_level;
        for tee in &self.tee_files {
            if tee.filter > max_dqcsim_verbosity {
                max_dqcsim_verbosity = tee.filter;
            }
        }

        // Clamp the verbosities of the sources.
        if self.dqcsim_level > max_dqcsim_verbosity {
            self.dqcsim_level = max_dqcsim_verbosity.clone();
        }
        for plugin in &mut self.plugins {
            if plugin.nonfunctional.verbosity > max_dqcsim_verbosity {
                plugin.nonfunctional.verbosity = max_dqcsim_verbosity.clone();
            }
        }
    }
}

impl Default for SimulatorConfiguration {
    /// Generates a default configuration.
    ///
    /// Note that the plugins vector still needs to be populated with at least
    /// two plugins.
    fn default() -> SimulatorConfiguration {
        SimulatorConfiguration {
            seed: Seed::default(),
            stderr_level: LoglevelFilter::Info,
            tee_files: vec![],
            dqcsim_level: LoglevelFilter::Info,
            plugins: vec![],
        }
    }
}
