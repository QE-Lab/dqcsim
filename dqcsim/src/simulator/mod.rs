use crate::{
    configuration::{ArbCmd, ArbData, PluginConfiguration, Seed, SimulatorConfiguration},
    debug, fatal,
    log::thread::LogThread,
    plugin::Plugin,
    trace,
};
use failure::{bail, format_err, Error};
use std::convert::{AsMut, AsRef};

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
    pub fn try_from(mut configuration: SimulatorConfiguration) -> Result<Simulator, Error> {
        // Check and optimize the configuration.
        configuration.check_plugin_list()?;
        configuration.optimize_loglevels();
        let configuration = configuration;

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

    /// Starts a program on the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    pub fn start(&mut self, args: impl Into<ArbData>) -> Result<(), Error> {
        // TODO
        bail!("Not yet implemented")
    }

    /// Waits for the accelerator to finish its current program.
    ///
    /// When this succeeds, the return value of the accelerator's `run()`
    /// function is returned.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    pub fn wait(&mut self) -> Result<ArbData, Error> {
        // TODO
        bail!("Not yet implemented")
    }

    /// Sends a message to the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    pub fn send(&mut self, args: impl Into<ArbData>) -> Result<(), Error> {
        // TODO
        bail!("Not yet implemented")
    }

    /// Waits for the accelerator to send a message to us.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    pub fn recv(&mut self) -> Result<ArbData, Error> {
        // TODO
        bail!("Not yet implemented")
    }

    /// Yields to the accelerator.
    ///
    /// The accelerator simulation runs until it blocks again. This is useful
    /// if you want an immediate response to an otherwise asynchronous call
    /// through the logging system or some communication channel outside of
    /// DQCsim's control.
    ///
    /// This function silently returns immediately if no asynchronous data was
    /// pending or if the simulator is waiting for something that has not been
    /// sent yet.
    pub fn yield_to_frontend(&mut self) -> Result<(), Error> {
        // TODO
        bail!("Not yet implemented")
    }

    /// Sends an `ArbCmd` message to one of the plugins, referenced by name.
    ///
    /// `ArbCmd`s are executed immediately after yielding to the simulator, so
    /// all pending asynchronous calls are flushed and executed *before* the
    /// `ArbCmd`.
    pub fn arb(&mut self, name: impl AsRef<str>, cmd: impl Into<ArbCmd>) -> Result<ArbData, Error> {
        let name = name.as_ref();
        for (idx, plugin) in self.pipeline.iter().enumerate() {
            if plugin.name() == name {
                return self.arb_idx(idx as isize, cmd);
            }
        }
        bail!("Plugin {} not found", name)
    }

    /// Sends an `ArbCmd` message to one of the plugins, referenced by index.
    ///
    /// The frontend always has index 0. 1 through N are used for the operators
    /// in front to back order (where N is the number of operators). The
    /// backend is at index N+1.
    ///
    /// Python-style negative indices are supported. That is, -1 can be used to
    /// refer to the backend, -2 to the last operator, and so on.
    ///
    /// `ArbCmd`s are executed immediately after yielding to the simulator, so
    /// all pending asynchronous calls are flushed and executed *before* the
    /// `ArbCmd`.
    pub fn arb_idx(&mut self, index: isize, cmd: impl Into<ArbCmd>) -> Result<ArbData, Error> {
        let mut index = index;
        let n_plugins = self.pipeline.len();
        if index < 0 {
            index += n_plugins as isize;
            if index < 0 {
                bail!("Index {} out of range", index);
            }
        }
        let index = index as usize;
        if index >= n_plugins {
            bail!("Index {} out of range", index);
        }
        self.yield_to_frontend()?;
        self.pipeline[index].arb(cmd)
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        trace!("Dropping Simulation.");
    }
}
