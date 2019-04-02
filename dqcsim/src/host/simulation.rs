//! Simulation instance.

use crate::{
    common::{
        error::{err, inv_arg, inv_op, Result},
        log::thread::LogThread,
        protocol::SimulatorToPlugin,
        types::{ArbCmd, ArbData, PluginMetadata},
    },
    host::{accelerator::Accelerator, configuration::Seed, plugin::Plugin},
    trace,
};
use std::collections::VecDeque;

/// Type alias for a pipeline of Plugin trait objects.
pub type Pipeline = Vec<Box<dyn Plugin>>;

/// Tracks the state of a Simulation.
#[derive(Debug)]
pub enum SimulationState {
    /// Simulation pipeline is idle.
    Idle,
    /// start() was called, but was not yet forwarded to the front-end.
    Pending,
    /// yield() returned, but the simulation program has not returned yet.
    Blocked,
    /// The simulation program has returned, but wait() has not yet been
    /// called
    Zombie,
}

/// Simulation instance.
///
///
#[derive(Debug)]
pub struct Simulation {
    state: SimulationState,
    pipeline: Pipeline,
    queue: VecDeque<SimulatorToPlugin>,
    seed: Seed,
}

impl Simulation {
    /// Constructs a Simulation from a collection of PluginInstance and a random seed.
    pub fn new(mut pipeline: Pipeline, seed: Seed, logger: &LogThread) -> Result<Simulation> {
        trace!("Constructing Simulation");
        if pipeline.len() < 2 {
            inv_arg("Simulation must consist of at least a frontend and backend")?
        }

        // Spawn the plugins.
        let (_, errors): (_, Vec<_>) = pipeline
            .iter_mut()
            .map(|plugin| plugin.spawn(logger))
            .partition(Result::is_ok);
        if !errors.is_empty() {
            err("Failed to spawn plugin(s)")?
        }

        // Initialize the plugins.
        let mut downstream = None;
        let (_, errors): (_, Vec<_>) = pipeline
            .iter_mut()
            .rev()
            .map(|plugin| {
                let res = plugin.init(logger, &downstream)?;
                downstream = res.upstream;
                Ok(())
            })
            .partition(Result::is_ok);
        if !errors.is_empty() {
            err("Failed to initialize plugin(s)")?
        }

        Ok(Simulation {
            state: SimulationState::Idle,
            pipeline,
            queue: VecDeque::new(),
            seed,
        })
    }

    /// Returns a mutable reference to the pipeline.
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        self.pipeline.as_mut()
    }

    #[allow(clippy::borrowed_box)]
    pub fn frontend(&self) -> &Box<dyn Plugin> {
        unsafe { self.pipeline.get_unchecked(0) }
    }
    #[allow(clippy::borrowed_box)]
    pub fn frontend_mut(&mut self) -> &mut Box<dyn Plugin> {
        unsafe { self.pipeline.get_unchecked_mut(0) }
    }

    pub fn abort(&mut self, _graceful: bool) -> Result<()> {
        unimplemented!()
    }

    /// Starts a program on the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    pub fn start(&mut self, _args: impl Into<ArbData>) -> Result<()> {
        unimplemented!()
    }

    /// Waits for the accelerator to finish its current program.
    ///
    /// When this succeeds, the return value of the accelerator's `run()`
    /// function is returned.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    pub fn wait(&mut self) -> Result<ArbData> {
        unimplemented!()
    }

    /// Sends a message to the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    pub fn send(&mut self, _args: impl Into<ArbData>) -> Result<()> {
        unimplemented!()
    }

    /// Waits for the accelerator to send a message to us.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    pub fn recv(&mut self) -> Result<ArbData> {
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
    }
}

impl Accelerator for Simulation {
    /// Starts a program on the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    fn start(&mut self, args: impl Into<ArbData>) -> Result<()> {
        Ok(())
    }

    /// Waits for the accelerator to finish its current program.
    ///
    /// When this succeeds, the return value of the accelerator's `run()`
    /// function is returned.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    fn wait(&mut self) -> Result<ArbData> {
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
        // TODO: matthijs send abort to all plugins. Then wait and collect exit.
    }
}
