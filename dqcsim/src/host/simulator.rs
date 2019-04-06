//! Simulator driver.

use crate::{
    common::{error::Result, log::thread::LogThread},
    host::{
        configuration::SimulatorConfiguration, plugin::Plugin, reproduction::Reproduction,
        simulation::Simulation,
    },
    trace, warn,
};

/// Simulator driver instance.
///
/// A Simulator instance wraps around a [`Simulation`] run and a [`LogThread`].
/// Its behavior is defined by a [`SimulatorConfiguration`]. The Simulator is
/// used to spawn the [`LogThread`] and construct and initialize the
/// [`Pipeline`] of the [`Simulation`].
///
/// After construction, users should directly interact with the [`Simulation`]
/// through the public [`simulation field`].
///
/// When the Simulator gets dropped it will ensure the [`Simulation`] gets
/// dropped before the [`LogThread`].
///
/// [`SimulatorConfiguration`]: ../configuration/struct.SimulatorConfiguration.html
/// [`Simulation`]: ../simulation/struct.Simulation.html
/// [`simulation field`]: ./struct.Simulator.html#structfield.simulation
/// [`Pipeline`]: ../simulation/struct.Pipeline.html
/// [`LogThread`]: ../log/thread/struct.LogThread.html
#[derive(Debug)]
pub struct Simulator {
    /// LogThread used by this Simulator for logging.
    log_thread: LogThread,

    /// The Simulation driven by this Simulator.
    pub simulation: Simulation,
}

impl Simulator {
    /// Constructs a Simulator driver.
    ///
    /// Spawns the log thread and constructs and initializes the inner
    /// Simulation instance.
    /// Returns the Simulator driver instance.
    pub fn new(mut configuration: SimulatorConfiguration) -> Result<Simulator> {
        // Check configuration.
        configuration.check_plugin_list()?;
        configuration.optimize_loglevels();

        // Try to build the reproduction logger.
        let reproduction = configuration
            .reproduction_path_style
            .map(|_| Reproduction::new_logger(&configuration));

        // Spawn log thread.
        let log_thread = LogThread::spawn(
            "dqcsim",
            configuration.dqcsim_level,
            configuration.stderr_level,
            configuration.log_callback,
            configuration.tee_files,
        )?;

        // Now that we can log, report any failures to create the reproduction
        // logger as a warning.
        let reproduction = match reproduction {
            Some(Ok(r)) => Some(r),
            Some(Err(e)) => {
                warn!("Failed to construct reproduction logger: {}", e.to_string());
                warn!("Therefore, you will not be able to reproduce this run on the command line later.");
                None
            }
            None => None,
        };

        // Construct plugin pipeline.
        let pipeline: Vec<Box<dyn Plugin>> = configuration
            .plugins
            .into_iter()
            .map(|plugin| plugin.instantiate())
            .collect();

        // Construct simulation.
        let simulation = Simulation::new(pipeline, configuration.seed, reproduction, &log_thread)?;

        Ok(Simulator {
            log_thread,
            simulation,
        })
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        trace!("Dropping Simulator");

        // Drain the simulation pipeline to drop the Plugin instances before
        // dropping the log thread.
        self.simulation.drop_plugins();
    }
}
