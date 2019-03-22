//! Simulator driver.
//!
//!

use crate::{
    common::{
        error::{inv_arg, Result},
        log::thread::LogThread,
    },
    host::{
        configuration::{PluginType, SimulatorConfiguration},
        simulation::Simulation,
    },
    trace,
};

/// Simulator instance.
///
/// A [`Simulator`] instance wraps around a [`Simulation`] run and a
/// [`LogThread`]. Its behavior is defined by a [`SimulatorConfiguration`].
///
/// # Example
///
/// ```rust
/// use dqcsim::{
///     host::{
///         configuration::SimulatorConfiguration,
///         simulator::Simulator,
///     },
///     common::error::Result,
/// };
///
/// // Note this is an example, the default configuration is not usable.
/// let configuration = SimulatorConfiguration::default();
///
/// let simulator: Result<Simulator> = Simulator::try_from(configuration);
///
/// ```
///
/// [`Simulator`]: ./struct.Simulator.html
/// [`SimulatorConfiguration`]: ../configuration/struct.SimulatorConfiguration.html
/// [`Simulation`]: ../simulation/struct.Simulation.html
/// [`LogThread`]: ../log/thread/struct.LogThread.html
#[derive(Debug)]
pub struct Simulator {
    /// LogThread used by this Simulator for logging.
    log_thread: LogThread,

    /// The Simulation running in this Simulator.
    /// Wrapped in an option container to control Drop order of the
    /// Simulator fields. The Simulation is always dropped before
    /// the LogThread.
    simulation: Option<Simulation>,
}

impl AsRef<Simulation> for Simulator {
    fn as_ref(&self) -> &Simulation {
        self.simulation.as_ref().unwrap()
    }
}

impl AsMut<Simulation> for Simulator {
    fn as_mut(&mut self) -> &mut Simulation {
        self.simulation.as_mut().unwrap()
    }
}

impl Simulator {
    /// Construct a Simulator instance from a SimulatorConfiguration.
    ///
    /// Returns the Simulator instance or an Error.
    /// Potential errors causes are related to spawning the LogThread and
    /// constructing the Simulation.
    pub fn new(mut configuration: SimulatorConfiguration) -> Result<Simulator> {
        Simulator::optimize_loglevels(&mut configuration);
        Simulator::check_plugin_list(&mut configuration)?;
        dbg!(&configuration);
        let mut sim = Simulator::try_from(configuration)?;
        sim.init()?;
        Ok(sim)
    }

    pub fn try_from(configuration: SimulatorConfiguration) -> Result<Simulator> {
        let log_thread = LogThread::spawn(
            "dqcsim",
            configuration.dqcsim_level,
            configuration.stderr_level,
            configuration.log_callback,
            configuration.tee_files,
        )?;

        trace!("Constructing Simulator");

        let simulation = Simulation::new(configuration.plugins, configuration.seed)?;

        Ok(Simulator {
            log_thread,
            simulation: Some(simulation),
        })
    }

    /// Initalize the Simulator.
    ///
    /// Initialize the [`Simulator`] by initializing the plugins in the
    /// [`Simulation`].
    pub fn init(&mut self) -> Result<()> {
        trace!("Initialize Simulator");
        let sender = self.log_thread.get_sender().unwrap();
        self.as_mut().spawn(sender)?;
        self.as_mut().init()
    }

    /// Optimizes the source verbosity levels, such that they are no more
    /// verbose than the most verbose sink.
    pub fn optimize_loglevels(configuration: &mut SimulatorConfiguration) {
        // Find the verbosity of the most verbose sink.
        let mut max_dqcsim_verbosity = configuration.stderr_level;
        for tee in &configuration.tee_files {
            if tee.filter > max_dqcsim_verbosity {
                max_dqcsim_verbosity = tee.filter;
            }
        }
        if let Some(cb) = configuration.log_callback.as_ref() {
            if cb.filter > max_dqcsim_verbosity {
                max_dqcsim_verbosity = cb.filter;
            }
        }

        // Clamp the verbosities of the sources.
        if configuration.dqcsim_level > max_dqcsim_verbosity {
            configuration.dqcsim_level = max_dqcsim_verbosity;
        }
        for plugin in &mut configuration.plugins {
            if plugin.nonfunctional.verbosity > max_dqcsim_verbosity {
                plugin.nonfunctional.verbosity = max_dqcsim_verbosity;
            }
        }
    }

    /// Verifies that the plugins are specified correctly.
    ///
    /// This checks that there is exactly one frontend and exactly one backend.
    /// If this is true but they're not in the right place, they are silently
    /// moved. This also ensures that there are no duplicate plugin names, and
    /// auto-names empty plugin names.
    fn check_plugin_list(configuration: &mut SimulatorConfiguration) -> Result<()> {
        // Check and fix frontend.
        let mut frontend_idx = None;
        for (i, plugin) in configuration.plugins.iter().enumerate() {
            if let PluginType::Frontend = plugin.specification.typ {
                if frontend_idx.is_some() {
                    inv_arg("duplicate frontend")?
                } else {
                    frontend_idx = Some(i);
                }
            }
        }
        match frontend_idx {
            Some(0) => (),
            Some(x) => {
                let plugin = configuration.plugins.remove(x);
                configuration.plugins.insert(0, plugin);
            }
            None => inv_arg("missing frontend")?,
        }

        // Check and fix backend.
        let mut backend_idx = None;
        for (i, plugin) in configuration.plugins.iter().enumerate() {
            if let PluginType::Backend = plugin.specification.typ {
                if backend_idx.is_some() {
                    inv_arg("duplicate backend")?
                } else {
                    backend_idx = Some(i);
                }
            }
        }
        match backend_idx {
            Some(x) => {
                if x != configuration.plugins.len() - 1 {
                    let plugin = configuration.plugins.remove(x);
                    configuration.plugins.push(plugin);
                }
            }
            None => inv_arg("missing backend")?,
        }

        // Auto-name plugins and check for conflicts.
        let mut names = std::collections::HashSet::new();
        for (i, plugin) in configuration.plugins.iter_mut().enumerate() {
            if plugin.name == "" {
                plugin.name = match plugin.specification.typ {
                    PluginType::Frontend => "front".to_string(),
                    PluginType::Operator => format!("op{}", i),
                    PluginType::Backend => "back".to_string(),
                }
            }
            if !names.insert(&plugin.name) {
                inv_arg(format!("duplicate plugin name '{}'", plugin.name))?;
            }
        }

        Ok(())
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        trace!("Dropping Simulator");

        // Drop the Simulation to allow the LogThread to outlive Simulation deconstruction.
        self.simulation.take();
    }
}

#[cfg(test)]
mod tests {
    use super::Simulator;
    use crate::host::configuration::{
        PluginConfiguration, PluginSpecification, PluginType, SimulatorConfiguration,
    };

    #[test]
    fn default_configuration() {
        // Default SimulatorConfiguration is not supposed to work.
        let simulator = Simulator::try_from(SimulatorConfiguration::default());
        assert!(simulator.is_err());
        assert_eq!(
            format!("{}", simulator.err().unwrap()),
            "Invalid argument: Simulation must consist of at least a frontend and backend"
        );
    }

    #[test]
    fn frontend_backend() {
        let mut configuration = SimulatorConfiguration::default();

        let frontend = PluginConfiguration::new(
            "frontend",
            PluginSpecification::from_sugar("/bin/sh", PluginType::Frontend).unwrap(),
        );
        let backend = PluginConfiguration::new(
            "backend",
            PluginSpecification::from_sugar("/bin/sh", PluginType::Backend).unwrap(),
        );

        configuration.plugins.push(frontend);
        configuration.plugins.push(backend);

        let simulator = Simulator::try_from(configuration);
        assert!(simulator.is_ok());
    }
}
