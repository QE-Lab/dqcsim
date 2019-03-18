use crate::{
    configuration::{PluginConfiguration, PluginType, Seed},
    log::{tee_file::TeeFile, LoglevelFilter, Record},
};
use failure::{Error, Fail};
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
    pub callback: Box<dyn Fn(&Record)>,

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

/// Error structure used for reporting simulator configuration errors.
#[derive(Debug, Fail, PartialEq)]
enum SimulatorConfigurationError {
    #[fail(display = "Duplicate frontend plugin")]
    DuplicateFrontend,
    #[fail(display = "Duplicate backend plugin")]
    DuplicateBackend,
    #[fail(display = "Missing frontend plugin")]
    MissingFrontend,
    #[fail(display = "Missing backend plugin")]
    MissingBackend,
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
        if let Some(cb) = self.log_callback.as_ref() {
            if cb.filter > max_dqcsim_verbosity {
                max_dqcsim_verbosity = cb.filter;
            }
        }

        // Clamp the verbosities of the sources.
        if self.dqcsim_level > max_dqcsim_verbosity {
            self.dqcsim_level = max_dqcsim_verbosity;
        }
        for plugin in &mut self.plugins {
            if plugin.nonfunctional.verbosity > max_dqcsim_verbosity {
                plugin.nonfunctional.verbosity = max_dqcsim_verbosity;
            }
        }
    }

    /// Verifies that the plugins are specified correctly.
    ///
    /// This checks that there is exactly one frontend and exactly one backend.
    /// If this is true but they're not in the right place, they are silently
    /// moved.
    pub fn check_plugin_list(&mut self) -> Result<(), Error> {
        // Check and fix frontend.
        let mut frontend_idx = None;
        for (i, item) in self.plugins.iter().enumerate() {
            if let PluginType::Frontend = item.specification.typ {
                if frontend_idx.is_some() {
                    Err(SimulatorConfigurationError::DuplicateFrontend)?
                } else {
                    frontend_idx = Some(i);
                }
            }
        }
        match frontend_idx {
            Some(0) => (),
            Some(x) => {
                let plugin = self.plugins.remove(x);
                self.plugins.insert(0, plugin);
            }
            None => Err(SimulatorConfigurationError::MissingFrontend)?,
        }

        // Check and fix backend.
        let mut backend_idx = None;
        for (i, item) in self.plugins.iter().enumerate() {
            if let PluginType::Backend = item.specification.typ {
                if backend_idx.is_some() {
                    Err(SimulatorConfigurationError::DuplicateBackend)?
                } else {
                    backend_idx = Some(i);
                }
            }
        }
        match backend_idx {
            Some(x) => {
                if x != self.plugins.len() - 1 {
                    let plugin = self.plugins.remove(x);
                    self.plugins.push(plugin);
                }
            }
            None => Err(SimulatorConfigurationError::MissingBackend)?,
        }

        Ok(())
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
            log_callback: None,
            dqcsim_level: LoglevelFilter::Info,
            plugins: vec![],
        }
    }
}
