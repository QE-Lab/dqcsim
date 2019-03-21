use crate::{
    common::log::{tee_file::TeeFile, LoglevelFilter, Record},
    host::configuration::{PluginConfiguration, Seed},
};
use serde::{Deserialize, Serialize};

/// Log callback function structure.
///
/// Note the lack of derives; they don't play well with `Box<dyn Fn...>`...
/// I wonder why. That's primarily why this struct is defined outside
/// `SimulatorConfiguration`.
pub struct LogCallback {
    /// The callback function to call.
    ///
    /// The sole argument is the log message record.
    pub callback: Box<dyn Fn(&Record) + Send>,

    /// Verbosity level for calling the log callback function.
    pub filter: LoglevelFilter,
}

impl std::fmt::Debug for LogCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "LogCallback {{ callback: <...>, filter: {:?} }}",
            self.filter
        )
    }
}

/// The complete configuration for a DQCsim run.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulatorConfiguration {
    /// The random seed for the simulation.
    pub seed: Seed,

    /// The verbosity for logging messages to stderr.
    pub stderr_level: LoglevelFilter,

    /// Logs messages to the specified file in addition to stderr. level sets
    /// the minimum importance for a message to be logged to this file.
    pub tee_files: Vec<TeeFile>,

    /// Optional log callback function.
    #[serde(skip)]
    pub log_callback: Option<LogCallback>,

    /// The verbosity for DQCsim itself.
    pub dqcsim_level: LoglevelFilter,

    /// The plugin configurations, from front to back.
    pub plugins: Vec<PluginConfiguration>,
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
            log_callback: None,
            dqcsim_level: LoglevelFilter::Info,
            plugins: vec![],
        }
    }
}
