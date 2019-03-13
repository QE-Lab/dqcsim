use crate::{
    log::{debug, fatal, info, thread::LogThread, trace, LevelFilter},
    plugin::{Plugin, PluginConfig},
    util::signal::notify,
};
use failure::{bail, Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SimulationOpt {
    #[structopt(short = "l", long = "loglevel")]
    pub loglevel: Option<LevelFilter>,
    #[structopt(raw(required = "true", min_values = "2"))]
    pub plugins: Vec<PluginConfig>,
}

pub struct Simulator {
    pub logger: LogThread,
    pub simulation: Simulation,
}

impl Simulator {
    pub fn new(opt: SimulationOpt) -> Result<Simulator, Error> {
        // Spawn log thread here so it outlives the Simulation instance. This
        // allows debugging deconstruction as Drop goes outer to inner recursively.
        let logger = LogThread::spawn(opt.loglevel.unwrap_or(LevelFilter::Trace));

        match Simulation::new(opt, &logger) {
            Err(err) => {
                bail!("Simulation construction failed: {}", err);
            }
            Ok(simulation) => {
                simulation.init();
                simulation.abort();
                Ok(Simulator { logger, simulation })
            }
        }
    }
}

pub struct Simulation {
    pipeline: Vec<Plugin>,
}

impl Simulation {
    pub fn new(opt: SimulationOpt, logger: &LogThread) -> Result<Simulation, Error> {
        info!(
            "Running {} v{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        let _signal = notify(&[
            signal_hook::SIGTERM,
            signal_hook::SIGINT,
            signal_hook::SIGQUIT,
        ])?;

        let (plugins, errors): (Vec<_>, Vec<_>) = opt
            .plugins
            .into_iter()
            .rev()
            .map(|config: PluginConfig| Plugin::new(config, &logger))
            .partition(Result::is_ok);

        let pipeline: Vec<Plugin> = plugins.into_iter().map(Result::unwrap).collect();

        if !errors.is_empty() {
            pipeline.iter().for_each(|plugin| plugin.abort());

            errors.into_iter().map(Result::unwrap_err).for_each(|err| {
                fatal!("Failed to construct plugin: {}", err);
            });

            bail!("Failed to start simulation.");
        }

        Ok(Simulation { pipeline })
    }

    pub fn init(&self) {
        debug!("Initialize plugins.");
        let mut iterator = self.pipeline.iter().skip(1);
        self.pipeline[0].init(None, &mut iterator);
    }

    pub fn abort(&self) {
        trace!("Aborting simulation.");
        self.pipeline.iter().for_each(|plugin| plugin.abort());
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        trace!("Dropping Simulation.");
    }
}
