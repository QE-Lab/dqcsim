use crate::{
    configuration::{PluginConfiguration, Seed, SimulatorConfiguration},
    debug, fatal,
    log::thread::LogThread,
    plugin::Plugin,
    trace,
};
use failure::{bail, format_err, Error};

/// Simulator instance.
///
/// Wraps around a [`Simulation`] and a [`LogThread`].
///
/// [`Simulation`]: ./struct.Simulation.html
/// [`LogThread`]: ../log/thread/struct.LogThread.html
#[derive(Debug)]
pub struct Simulator {
    pub log_thread: LogThread,
    pub simulation: Option<Simulation>,
}

impl Drop for Simulator {
    fn drop(&mut self) {
        trace!("Dropping Simulator");

        // This forces the Simulation to be dropped before the Logger.
        self.simulation = None;
    }
}

impl Simulator {
    pub fn try_from(configuration: SimulatorConfiguration) -> Result<Simulator, Error> {
        // Spawn log thread here so it outlives the Simulation instance. This
        // allows debugging deconstruction as Drop goes outer to inner recursively.
        let log_thread = LogThread::spawn(configuration.stderr_level, configuration.dqcsim_level)?;

        let mut simulation =
            Simulation::new(configuration.plugins, configuration.seed, &log_thread)?;
        match simulation.init() {
            Err(_) => {
                simulation.abort(false)?;
                fatal!("Simulation init failed");
                bail!("Simulation init failed")
            }
            Ok(_) => Ok(Simulator {
                log_thread,
                simulation: Some(simulation),
            }),
        }
    }
    fn simulation_mut(&mut self) -> &mut Simulation {
        self.simulation.as_mut().unwrap()
    }
    pub fn abort(&mut self) -> Result<(), Error> {
        // Graceful termination
        self.simulation_mut().abort(true)
    }
    pub fn kill(&mut self) -> Result<(), Error> {
        self.simulation_mut().abort(false)
    }
}

/// Simulation instance.
///
/// Contains a pipeline of [`Plugin`] instances.
///
/// [`Plugin`]: ../plugin/struct.Plugin.html
#[derive(Debug)]
pub struct Simulation {
    seed: Seed,
    pipeline: Vec<Plugin>,
}

impl Simulation {
    /// Constructs a Simulation based on a SimultationOpt.
    /// Requires a [`LogThread`] to be available.
    ///
    /// [`LogThread`]: ../log/thread/struct.LogThread.html
    pub fn new(
        plugins: Vec<PluginConfiguration>,
        seed: Seed,
        logger: &LogThread,
    ) -> Result<Simulation, Error> {
        // let signal = notify(&[
        //     signal_hook::SIGTERM,
        //     signal_hook::SIGINT,
        //     signal_hook::SIGQUIT,
        // ])?;

        let (plugins, errors): (Vec<_>, Vec<_>) = plugins
            .into_iter()
            .rev()
            .map(|configuration: PluginConfiguration| Plugin::new(configuration, &logger))
            .partition(Result::is_ok);

        let mut pipeline: Vec<Plugin> = plugins.into_iter().map(Result::unwrap).collect();

        if !errors.is_empty() {
            pipeline.iter_mut().for_each(|plugin| plugin.abort(false));

            errors.into_iter().map(Result::unwrap_err).for_each(|err| {
                fatal!("Failed to construct plugin: {}", err);
            });

            bail!("Failed to start simulation.")
        }

        Ok(Simulation { seed, pipeline })
    }

    pub fn init(&self) -> Result<(), Error> {
        debug!("Initialize plugins.");
        self.pipeline
            .first()
            .ok_or_else(|| format_err!("Plugin missing"))?
            .init(None, self.pipeline.iter().skip(1).by_ref())?;
        debug!("Initialization of plugins succesful.");
        Ok(())
    }

    /// Abort the simulation.
    ///
    /// Graceful flag can be set to gracefully abort.
    /// Non-graceful termination should only be used in case of pre-
    /// initialization problems.
    pub fn abort(&mut self, graceful: bool) -> Result<(), Error> {
        trace!("Aborting simulation. (graceful: {})", graceful);
        self.pipeline
            .iter_mut()
            .for_each(|plugin| plugin.abort(graceful));
        Ok(())
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        trace!("Dropping Simulation.");
    }
}
