use crate::{
    log::{debug, fatal, thread::LogThread, trace, LevelFilter},
    plugin::{Plugin, PluginConfig},
};
use failure::{bail, format_err, Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SimulationOpt {
    #[structopt(short = "l", long = "loglevel")]
    pub loglevel: Option<LevelFilter>,
    #[structopt(raw(required = "true", min_values = "2"))]
    pub plugins: Vec<PluginConfig>,
}

/// Simulator instance.
///
/// Wraps around a [`Simulation`] and a [`LogThread`].
///
/// [`Simulation`]: ./struct.Simulation.html
/// [`LogThread`]: ../log/thread/struct.LogThread.html
#[derive(Debug)]
pub struct Simulator {
    pub logger: LogThread,
    pub simulation: Option<Simulation>,
}

impl Simulator {
    /// Constructs a Simulator instance based on a SimulationOpt.
    pub fn new(opt: SimulationOpt) -> Result<Simulator, Error> {
        // Spawn log thread here so it outlives the Simulation instance. This
        // allows debugging deconstruction as Drop goes outer to inner recursively.
        let logger = LogThread::spawn(opt.loglevel.unwrap_or(LevelFilter::Info))?;

        match Simulation::new(opt, &logger) {
            Err(err) => {
                fatal!("Simulation construction failed: {}", err);
                bail!("Simulation construction failed: {}", err);
            }
            Ok(mut simulation) => match simulation.init() {
                Err(_) => {
                    simulation.abort(false)?;
                    fatal!("Simulation init failed");
                    bail!("Simulation init failed")
                }
                Ok(_) => Ok(Simulator {
                    logger,
                    simulation: Some(simulation),
                }),
            },
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

impl Drop for Simulator {
    fn drop(&mut self) {
        trace!("Dropping Simulator");

        // This forces the Simulation to be dropped before the Logger.
        self.simulation = None;
    }
}

/// Simulation instance.
///
/// Contains a pipeline of [`Plugin`] instances.
///
/// [`Plugin`]: ../plugin/struct.Plugin.html
#[derive(Debug)]
pub struct Simulation {
    pipeline: Vec<Plugin>,
}

impl Simulation {
    /// Constructs a Simulation based on a SimultationOpt.
    /// Requires a [`LogThread`] to be available.
    ///
    /// [`LogThread`]: ../log/thread/struct.LogThread.html
    pub fn new(opt: SimulationOpt, logger: &LogThread) -> Result<Simulation, Error> {
        // let signal = notify(&[
        //     signal_hook::SIGTERM,
        //     signal_hook::SIGINT,
        //     signal_hook::SIGQUIT,
        // ])?;

        let (plugins, errors): (Vec<_>, Vec<_>) = opt
            .plugins
            .into_iter()
            .rev()
            .map(|config: PluginConfig| Plugin::new(config, &logger))
            .partition(Result::is_ok);

        let mut pipeline: Vec<Plugin> = plugins.into_iter().map(Result::unwrap).collect();

        if !errors.is_empty() {
            pipeline.iter_mut().for_each(|plugin| plugin.abort(false));

            errors.into_iter().map(Result::unwrap_err).for_each(|err| {
                fatal!("Failed to construct plugin: {}", err);
            });

            bail!("Failed to start simulation.")
        }

        Ok(Simulation { pipeline })
    }

    pub fn init(&self) -> Result<(), Error> {
        debug!("Initialize plugins.");
        self.pipeline
            .first()
            .ok_or(format_err!("Plugin missing"))?
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
