//! Simulation instance.

use crate::{
    common::{
        error::{err, inv_arg, inv_op, Result},
        log::thread::LogThread,
        types::{ArbCmd, ArbData},
    },
    host::{configuration::Seed, plugin::Plugin},
    trace,
};

pub type Pipeline = Vec<Box<dyn Plugin>>;

/// Simulation instance.
///
pub struct Simulation {
    pipeline: Pipeline,
    seed: Seed,
}

impl Simulation {
    /// Constructs a Simulation from a collection of PluginInstance and a random seed.
    pub fn new(pipeline: Pipeline, seed: Seed) -> Result<Simulation> {
        trace!("Constructing Simulation");
        if pipeline.len() < 2 {
            inv_arg("Simulation must consist of at least a frontend and backend")?
        }

        Ok(Simulation { seed, pipeline })
    }

    /// Returns a mutable reference to the pipeline.
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        self.pipeline.as_mut()
    }

    /// Spawn the plugins in the Simulation.
    ///
    /// Required to run before [`init`].
    pub fn spawn(&mut self, logger: &LogThread) -> Result<()> {
        let (_, errors): (_, Vec<_>) = self
            .pipeline
            .iter_mut()
            .map(|plugin| plugin.spawn(logger))
            .partition(Result::is_ok);
        if !errors.is_empty() {
            err("Failed to spawn plugin(s)")?
        } else {
            Ok(())
        }
    }

    pub fn abort(&mut self, graceful: bool) -> Result<()> {
        unimplemented!()
    }

    /// Initialize the Simulation.
    ///
    /// Initialize the [`Simulation`] by spawning the plugin processes and
    /// initializing the plugins.
    pub fn init(&mut self, logger: &LogThread) -> Result<()> {
        trace!("Initialize Simulation");
        self.pipeline.iter().rev().next().unwrap();
        // .init(None, self.pipeline.iter().rev().skip(1).by_ref())?;
        Ok(())
    }

    /// Starts a program on the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    pub fn start(&mut self, args: impl Into<ArbData>) -> Result<()> {
        Ok(())
    }

    /// Waits for the accelerator to finish its current program.
    ///
    /// When this succeeds, the return value of the accelerator's `run()`
    /// function is returned.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    pub fn wait(&mut self) -> Result<ArbData> {
        Ok(ArbData::default())
    }

    /// Sends a message to the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    #[allow(unused)] // TODO: remove <--
    pub fn send(&mut self, args: impl Into<ArbData>) -> Result<()> {
        // TODO
        inv_op("Not yet implemented")
    }

    /// Waits for the accelerator to send a message to us.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    pub fn recv(&mut self) -> Result<ArbData> {
        // TODO
        inv_op("Not yet implemented")
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
        inv_op("Not yet implemented")
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
        inv_arg(format!("Plugin {} not found", name))
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
                inv_arg(format!("Index {} out of range", index))?
            }
        }
        let index = index as usize;
        if index >= n_plugins {
            inv_arg(format!("Index {} out of range", index))?
        }
        self.yield_to_frontend()?;
        self.pipeline[index].arb(cmd)
    }
}
// /// Sends an `PluginInitializeRequest` to this plugin.
// pub fn init(
//     &self,
//     logger: &LogThread,
//     downstream: Option<String>,
// ) -> Result<PluginInitializeResponse> {
//     self.send(SimulatorToPlugin::Initialize(Box::new(
//         PluginInitializeRequest {
//             downstream,
//             configuration: self.configuration(),
//             log: logger.get_ipc_sender(),
//         },
//     )))?;
//
//     match self.recv()? {
//         PluginToSimulator::Initialized(response) => Ok(response),
//         PluginToSimulator::Failure(data) => err(data),
//         _ => inv_op("Unexpected response from plugin"),
//     }
// }
//
// /// Starts a program on the accelerator.
// ///
// /// This is an asynchronous call: nothing happens until `yield()`,
// /// `recv()`, or `wait()` is called.
// pub fn start(&mut self, args: impl Into<ArbData>) -> Result<()> {
//     assert_eq!(self.plugin_type(), PluginType::Frontend);
//     self.send(SimulatorToPlugin::RunRequest(FrontendRunRequest {
//         start: Some(args.into()),
//         messages: vec![],
//     }))?;
//     Ok(())
// }
//
// pub fn wait(&mut self) -> Result<ArbData> {
//     unimplemented!()
// }

impl Drop for Simulation {
    fn drop(&mut self) {
        trace!("Dropping Simulation");
        // TODO: matthijs send abort to all plugins. Then wait and collect exit.
    }
}
