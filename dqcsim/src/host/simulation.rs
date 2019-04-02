//! Simulation instance.

use crate::{
    common::{
        error::{err, inv_arg, inv_op, ErrorKind, Result},
        log::LogRecord,
        types::{ArbCmd, ArbData, PluginMetadata},
    },
    host::{
        accelerator::Accelerator,
        configuration::{PluginConfiguration, Seed},
        plugin::Plugin,
    },
    trace,
};
use ipc_channel::ipc::IpcSender;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SimulationState {
    /// Simulation pipeline is constructed.
    Constructed,
    /// Simulation spawned plugins in pipeline.
    Spawned,
    /// Simulation initalized plugins in pipeline.
    ///
    /// This includes upstream and downstream channel setup.
    Initialized,
}

/// Simulation instance.
///
/// Contains a pipeline of [`Plugin`] instances.
///
/// [`Plugin`]: ../plugin/struct.Plugin.html
#[derive(Debug)]
pub struct Simulation {
    state: SimulationState,
    seed: Seed,
    pipeline: Vec<Plugin>,
}

impl Simulation {
    /// Constructs a [`Simulation`] based on the provided [`PluginConfiguration`]s.
    pub fn new(plugins: Vec<PluginConfiguration>, seed: Seed) -> Result<Simulation> {
        trace!("Constructing Simulation");
        if plugins.len() < 2 {
            inv_arg("Simulation must consist of at least a frontend and backend")?
        }

        let (plugins, errors): (Vec<_>, Vec<_>) = plugins
            .into_iter()
            .rev()
            .map(Plugin::try_from)
            .partition(Result::is_ok);

        let pipeline: Vec<Plugin> = plugins.into_iter().map(Result::unwrap).collect();

        if !errors.is_empty() {
            Err(ErrorKind::Multiple(
                errors
                    .into_iter()
                    .map(|x| ErrorKind::Other(x.unwrap_err().to_string()))
                    .collect(),
            ))?
        }

        Ok(Simulation {
            state: SimulationState::Constructed,
            seed,
            pipeline,
        })
    }

    /// Spawn the plugins in the Simulation.
    ///
    /// Required to run before [`init`].
    pub fn spawn(&mut self, log_sender: crossbeam_channel::Sender<LogRecord>) -> Result<()> {
        match self.state {
            SimulationState::Constructed => {
                let (_, errors): (_, Vec<_>) = self
                    .pipeline
                    .iter_mut()
                    .map(|plugin| plugin.spawn(log_sender.clone()))
                    .partition(Result::is_ok);
                if !errors.is_empty() {
                    err("Failed to spawn plugin(s)")?
                }
            }
            _ => {
                inv_op(format!("spawn functions requires the simulation to be in the Constructed state. Current state: {:?}", self.state))?
            }
        }
        self.state = SimulationState::Spawned;
        Ok(())
    }

    /// Initialize the Simulation.
    ///
    /// Initialize the [`Simulation`] by spawning the plugin processes and
    /// initializing the plugins.
    pub fn init(&mut self, log: IpcSender<LogRecord>) -> Result<()> {
        trace!("Initialize Simulation");
        match self.state {
            SimulationState::Spawned => {
                self.pipeline
                    .first()
                    .unwrap()
                    .init(None, self.pipeline.iter().skip(1).by_ref(), log)?;
                    self.state = SimulationState::Initialized;
                    Ok(())
            },
            _ => {
                inv_op(format!("init functions requires the simulation to be in the Spawned state. Current state: {:?}", self.state))?
            }
        }
    }

    /// Abort the simulation.
    ///
    /// Graceful flag can be set to gracefully abort.
    /// Non-graceful termination should only be used in case of pre-
    /// initialization problems.
    pub fn abort(&mut self, _graceful: bool) -> Result<()> {
        // trace!("Aborting simulation. (graceful: {})", graceful);
        // self.pipeline
        //     .iter_mut()
        //     .for_each(|plugin| plugin.abort(graceful));
        Ok(())
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
    pub fn yield_to_frontend(&mut self) -> Result<()> {
        // TODO
        inv_op("not yet implemented")
    }

    /// Sends an `ArbCmd` message to one of the plugins, referenced by name.
    ///
    /// `ArbCmd`s are executed immediately after yielding to the simulator, so
    /// all pending asynchronous calls are flushed and executed *before* the
    /// `ArbCmd`.
    pub fn arb(&mut self, name: impl AsRef<str>, cmd: impl Into<ArbCmd>) -> Result<ArbData> {
        let name = name.as_ref();
        for (idx, plugin) in self.pipeline.iter().enumerate() {
            if plugin.name() == name {
                return self.arb_idx(idx as isize, cmd);
            }
        }
        inv_arg(format!("plugin {} not found", name))
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
    pub fn arb_idx(&mut self, index: isize, cmd: impl Into<ArbCmd>) -> Result<ArbData> {
        let mut index = index;
        let n_plugins = self.pipeline.len();
        if index < 0 {
            index += n_plugins as isize;
            if index < 0 {
                inv_arg(format!("index {} out of range", index))?
            }
        }
        let index = index as usize;
        if index >= n_plugins {
            inv_arg(format!("index {} out of range", index))?
        }
        self.yield_to_frontend()?;
        self.pipeline[index].arb(cmd)
    }

    /// Returns a reference to the metadata object belonging to the plugin
    /// referenced by instance name.
    pub fn get_metadata(&self, name: impl AsRef<str>) -> Result<&PluginMetadata> {
        let name = name.as_ref();
        for (idx, plugin) in self.pipeline.iter().enumerate() {
            if plugin.name() == name {
                return self.get_metadata_idx(idx as isize);
            }
        }
        inv_arg(format!("plugin {} not found", name))
    }

    /// Returns a reference to the metadata object belonging to the plugin
    /// referenced by index.
    pub fn get_metadata_idx(&self, index: isize) -> Result<&PluginMetadata> {
        let mut index = index;
        let n_plugins = self.pipeline.len();
        if index < 0 {
            index += n_plugins as isize;
            if index < 0 {
                inv_arg(format!("index {} out of range", index))?
            }
        }
        let index = index as usize;
        if index >= n_plugins {
            inv_arg(format!("index {} out of range", index))?
        }
        // TODO: get the metadata object
        inv_op("not yet implemented")
    }

    // /// Abort the simulation.
    // ///
    // /// Graceful flag can be set to gracefully abort.
    // /// Non-graceful termination should only be used in case of pre-
    // /// initialization problems.
    // pub fn abort(&mut self, graceful: bool) -> Result<(), Error> {
    //     trace!("Aborting simulation. (graceful: {})", graceful);
    //     self.pipeline
    //         .iter_mut()
    //         .for_each(|plugin| plugin.abort(graceful));
    //     Ok(())
    // }
}

impl Accelerator for Simulation {
    /// Starts a program on the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    fn start(&mut self, args: impl Into<ArbData>) -> Result<()> {
        if self.state != SimulationState::Initialized {
            inv_op(format!("init functions requires the simulation to be in the Initialized state. Current state: {:?}", self.state))?
        }
        Ok(())
    }

    /// Waits for the accelerator to finish its current program.
    ///
    /// When this succeeds, the return value of the accelerator's `run()`
    /// function is returned.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    fn wait(&mut self) -> Result<ArbData> {
        if self.state != SimulationState::Initialized {
            inv_op(format!("init functions requires the simulation to be in the Initialized state. Current state: {:?}", self.state))?
        }
        Ok(ArbData::default())
    }

    /// Sends a message to the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    fn send(&mut self, args: impl Into<ArbData>) -> Result<()> {
        // TODO
        inv_op("not yet implemented")
    }

    /// Waits for the accelerator to send a message to us.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    fn recv(&mut self) -> Result<ArbData> {
        // TODO
        inv_op("not yet implemented")
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        trace!("Dropping Simulation");

        // TODO: send abort to all plugins. Then wait and collect exit.
    }
}
