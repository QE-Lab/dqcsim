//! Simulator driver.

use crate::{
    common::{
        error::{inv_arg, Result},
        log::thread::LogThread,
    },
    host::{
        configuration::{PluginType, SimulatorConfiguration},
        plugin::{process::PluginProcess, Plugin},
        simulation::Simulation,
    },
    trace,
};

/// Simulator driver instance.
///
/// A [`Simulator`] instance wraps around a [`Simulation`] run and a
/// [`LogThread`]. Its behavior is defined by a [`SimulatorConfiguration`].
///
/// [`Simulator`]: ./struct.Simulator.html
/// [`SimulatorConfiguration`]: ../configuration/struct.SimulatorConfiguration.html
/// [`Simulation`]: ../simulation/struct.Simulation.html
/// [`LogThread`]: ../log/thread/struct.LogThread.html
pub struct Simulator {
    /// LogThread used by this Simulator for logging.
    log_thread: LogThread,

    /// The Simulation pipeline running in this Simulator.
    pub simulation: Simulation,
}

impl Simulator {
    /// Construct a Simulator driver instance from a SimulatorConfiguration.
    ///
    /// Spawns the log thread and constructs the inner Simulation instance.
    /// Returns the Simulator driver instance.
    pub fn new(mut configuration: SimulatorConfiguration) -> Result<Simulator> {
        Simulator::check_plugin_list(&mut configuration)?;

        dbg!(&configuration);

        let log_thread = LogThread::spawn(
            "dqcsim",
            configuration.dqcsim_level,
            configuration.stderr_level,
            configuration.log_callback,
            configuration.tee_files,
        )?;

        trace!("Constructing Simulator");

        let simulation = Simulation::new(
            configuration
                .plugins
                .into_iter()
                // TODO: matthijs
                .map(PluginProcess::new)
                .map(Box::new)
                .map(|x| x as Box<dyn Plugin>)
                .collect(),
            configuration.seed,
        )?;

        Ok(Simulator {
            log_thread,
            simulation,
        })
    }

    /// Initalize the Simulator.
    ///
    /// Initialize the [`Simulator`] by spawning and initializing the plugins
    /// in the [`Simulation`].
    pub fn init(&mut self) -> Result<()> {
        trace!("Initialize Simulator");
        // let logger = &self.log_thread;
        // let (_, errors): (_, Vec<_>) = self
        //     .simulation
        //     .iter_mut()
        //     .map(|plugin| plugin.spawn(logger))
        //     .partition(Result::is_ok);
        // if !errors.is_empty() {
        //     err("Failed to spawn plugin(s)")?
        // } else {
        //     let mut downstream = None;
        //     for plugin in self.simulation.iter_mut().rev() {
        //         let res = plugin.init(logger, downstream)?;
        //         downstream = res.upstream;
        //     }
        //     Ok(())
        // }
        // let simulation = self.simulation.as_mut();
        self.simulation.spawn(&self.log_thread)?;
        self.simulation.init(&self.log_thread)
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

        // Drain the simulation pipeline to drop the Plugin instances before
        // dropping the log thread.
        self.simulation.pipeline_mut().drain(..);
    }
}
