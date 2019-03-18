//! Simulator driver.
//!
//!

use crate::{
    configuration::SimulatorConfiguration, error::Result, log::thread::LogThread,
    simulation::Simulation, trace,
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
///     configuration::SimulatorConfiguration,
///     simulator::Simulator,
///     error::Result,
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
    pub fn try_from(configuration: SimulatorConfiguration) -> Result<Simulator> {
        let log_thread = LogThread::spawn(
            configuration.stderr_level,
            configuration.dqcsim_level,
            configuration.log_callback,
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
        self.simulation_mut().spawn(sender)?;
        self.simulation_ref().init()
    }

    /// Returns reference to Simulation.
    fn simulation_ref(&self) -> &Simulation {
        self.simulation.as_ref().unwrap()
    }

    /// Returns mutable reference to Simulation.
    fn simulation_mut(&mut self) -> &mut Simulation {
        self.simulation.as_mut().unwrap()
    }

    // /// Abort the simulation. This sends a Request to all Plugins in the
    // /// Simulation to gracefully terminate.
    // pub fn abort(&mut self) -> Result<(), Error> {
    //     // Graceful termination
    //     self.simulation_mut().abort(true)
    // }
    //
    // /// Kill the simulation. Sends SIGKILL to all the Plugins in the
    // /// Simulation.
    // pub fn kill(&mut self) -> Result<(), Error> {
    //     self.simulation_mut().abort(false)
    // }
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
    use crate::configuration::{
        PluginConfiguration, PluginSpecification, PluginType, SimulatorConfiguration,
    };

    // #[test]
    // fn default_configuration() {
    //     // Default SimulatorConfiguration is not supposed to work.
    //     let simulator = Simulator::try_from(SimulatorConfiguration::default());
    //     assert!(simulator.is_err());
    //     assert_eq!(
    //         format!("{}", simulator.err().unwrap()),
    //         "Simulation consists of at least a frontend and backend"
    //     );
    // }

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
        let err = simulator.err().unwrap();
        eprintln!("{}", err);
    }
}
