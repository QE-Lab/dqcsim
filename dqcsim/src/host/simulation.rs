//! Simulation instance.

use crate::{
    checked_rpc,
    common::{
        error::{err, inv_arg, inv_op, Result},
        log::thread::LogThread,
        protocol::{FrontendRunRequest, PluginToSimulator},
        types::{ArbCmd, ArbData, PluginMetadata},
    },
    debug, error,
    host::{
        accelerator::Accelerator,
        configuration::Seed,
        plugin::Plugin,
        reproduction::{HostCall, Reproduction},
    },
    trace,
};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use std::collections::VecDeque;
use std::path::Path;

/// Type alias for a pipeline of Plugin trait objects.
pub type Pipeline = Vec<Box<dyn Plugin>>;

#[derive(Debug)]
struct InitializedPlugin {
    pub plugin: Box<dyn Plugin>,
    pub metadata: PluginMetadata,
}

/// Tracks the state of the simulated accelerator.
#[derive(Debug, PartialEq)]
enum AcceleratorState {
    /// The accelerator is idle.
    Idle,

    /// `start()` was called, but was not yet forwarded to the frontend. The
    /// contained value holds the argument to the `run()` frontend callback.
    StartPending(ArbData),

    /// yield() returned, but the `run()` frontend callback has not returned
    /// yet. We're not allowed to start a new program in this state.
    Blocked,

    /// The `run()` frontend callback has returned with the contained return
    /// value, but `wait()` has not yet been called.
    WaitPending(ArbData),
}

impl AcceleratorState {
    pub fn is_idle(&self) -> bool {
        &AcceleratorState::Idle == self
    }

    pub fn is_start_pending(&self) -> bool {
        if let AcceleratorState::StartPending(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_blocked(&self) -> bool {
        &AcceleratorState::Blocked == self
    }

    pub fn is_wait_pending(&self) -> bool {
        if let AcceleratorState::WaitPending(_) = self {
            true
        } else {
            false
        }
    }

    fn data(self) -> ArbData {
        match self {
            AcceleratorState::StartPending(x) => x,
            AcceleratorState::WaitPending(x) => x,
            _ => panic!("no data pending"),
        }
    }

    pub fn put_data(&mut self, data: ArbData) -> Result<()> {
        match self {
            AcceleratorState::Idle => {
                std::mem::replace(self, AcceleratorState::StartPending(data));
                Ok(())
            }
            AcceleratorState::StartPending(_) => inv_op("data is already pending"),
            AcceleratorState::Blocked => {
                std::mem::replace(self, AcceleratorState::WaitPending(data));
                Ok(())
            }
            AcceleratorState::WaitPending(_) => inv_op("data is already pending"),
        }
    }

    pub fn take_data(&mut self) -> Result<ArbData> {
        match self {
            AcceleratorState::Idle => inv_op("no data pending"),
            AcceleratorState::StartPending(_) => {
                Ok(std::mem::replace(self, AcceleratorState::Blocked).data())
            }
            AcceleratorState::Blocked => inv_op("no data pending"),
            AcceleratorState::WaitPending(_) => {
                Ok(std::mem::replace(self, AcceleratorState::Idle).data())
            }
        }
    }
}

/// Simulation instance.
#[derive(Debug)]
pub struct Simulation {
    /// The Plugin pipeline of this Simulation.
    pipeline: Vec<InitializedPlugin>,

    /// Tracks the state of the accelerator/frontend.
    state: AcceleratorState,

    /// Objects queued with `send()`, to be sent to the accelerator by the next
    /// yield.
    host_to_accelerator_data: VecDeque<ArbData>,

    /// Objects received from the accelerator, to be consumed using `recv()`.
    accelerator_to_host_data: VecDeque<ArbData>,

    /// Reproduction storage.
    reproduction_log: Option<Reproduction>,
}

impl Simulation {
    /// Constructs a Simulation from a collection of PluginInstance and a random seed.
    pub fn new(
        mut pipeline: Pipeline,
        seed: Seed,
        reproduction_log: Option<Reproduction>,
        logger: &LogThread,
    ) -> Result<Simulation> {
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
        let mut rng = ChaChaRng::seed_from_u64(seed.value);
        let (metadata, errors): (Vec<_>, Vec<_>) = pipeline
            .iter_mut()
            .rev()
            .map(|plugin| {
                let res = plugin.initialize(logger, &downstream, rng.next_u64())?;
                downstream = res.upstream;
                Ok(res.metadata)
            })
            .partition(Result::is_ok);

        // Check for initialization errors.
        if !errors.is_empty() {
            let mut messages = String::new();
            for e in errors {
                let e = e.unwrap_err();
                if messages.is_empty() {
                    messages = e.to_string();
                } else {
                    messages = format!("{}; {}", messages, e.to_string());
                }
            }
            err(format!("Failed to initialize plugin(s): {}", messages))?
        }

        // Tell downstream plugins to wait for a connection from upstream
        // plugins.
        let (_, errors): (Vec<_>, Vec<_>) = pipeline
            .iter_mut()
            .skip(1)
            .rev()
            .map(|plugin| plugin.accept_upstream())
            .partition(Result::is_ok);

        // Check for initialization errors.
        if !errors.is_empty() {
            let mut messages = String::new();
            for e in errors {
                let e = e.unwrap_err();
                if messages.is_empty() {
                    messages = e.to_string();
                } else {
                    messages = format!("{}; {}", messages, e.to_string());
                }
            }
            err(format!("Failed to initialize plugin(s): {}", messages))?
        }

        // Run the user intialization code.
        let (_, errors): (Vec<_>, Vec<_>) = pipeline
            .iter_mut()
            .rev()
            .map(|plugin| plugin.user_initialize())
            .partition(Result::is_ok);

        // Check for initialization errors.
        if !errors.is_empty() {
            let mut messages = String::new();
            for e in errors {
                let e = e.unwrap_err();
                if messages.is_empty() {
                    messages = e.to_string();
                } else {
                    messages = format!("{}; {}", messages, e.to_string());
                }
            }
            err(format!("Failed to initialize plugin(s): {}", messages))?
        }

        // Fix up the metadata vector.
        let metadata: Vec<_> = metadata.into_iter().map(Result::unwrap).rev().collect();

        let pipeline: Vec<_> = pipeline
            .into_iter()
            .zip(metadata.into_iter())
            .map(|(plugin, metadata)| InitializedPlugin { plugin, metadata })
            .collect();

        for (i, p) in pipeline.iter().enumerate() {
            debug!(
                "Plugin {} with instance name {} is {}",
                i,
                p.plugin.name(),
                p.metadata,
            );
        }

        Ok(Simulation {
            pipeline,
            state: AcceleratorState::Idle,
            host_to_accelerator_data: VecDeque::new(),
            accelerator_to_host_data: VecDeque::new(),
            reproduction_log,
        })
    }

    /// Drains the plugin pipeline so their drop() implementations get called.
    pub fn drop_plugins(&mut self) {
        trace!("Implicit yield() prior to dropping plugins...");
        if let Err(e) = self.internal_yield() {
            error!("Implicit yield to frontend failed: {}", e.to_string());
        }
        trace!("Dropping plugins...");
        for p in self.pipeline.drain(..) {
            let name = p.plugin.name();
            let _ = p;
            trace!("Dropped {}...", name);
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn accelerator(&self) -> &Box<dyn Plugin> {
        unsafe { &self.pipeline.get_unchecked(0).plugin }
    }

    #[allow(clippy::borrowed_box)]
    pub fn accelerator_mut(&mut self) -> &mut Box<dyn Plugin> {
        unsafe { &mut self.pipeline.get_unchecked_mut(0).plugin }
    }

    /// Internal function used to yield to the accelerator. This is called
    /// whenever we need to block to get data from the simulation.
    fn internal_yield(&mut self) -> Result<()> {
        // If a `start()` is pending, move the state to `Blocked` and send the
        // start command to the accelerator.
        let start = if self.state.is_start_pending() {
            Some(self.state.take_data().unwrap())
        } else {
            None
        };

        // Drain the pending messages into the appropriate data format for
        // transmission.
        let messages = self.host_to_accelerator_data.drain(..).collect();

        // Send the run RPC.
        let response = checked_rpc!(
            self.accelerator_mut(),
            FrontendRunRequest {
                start,
                messages,
            },
            expect RunResponse
        )?;

        // Queue up the messages sent to us by the accelerator.
        self.accelerator_to_host_data.extend(response.messages);

        // If we received a `run()` return value from accelerator, move to the
        // zombie state.
        if let Some(return_value) = response.return_value {
            if !self.state.is_blocked() {
                return err("Protocol error: unexpected run() return value");
            }
            self.state.put_data(return_value).unwrap();
        }

        Ok(())
    }

    /// Records a host call to the reproduction log, if we have one.
    fn record_host_call(&mut self, host_call: HostCall) {
        if let Some(log) = self.reproduction_log.as_mut() {
            debug!("recording host call to reproduction log: {:?}", &host_call);
            log.record(host_call);
        }
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
    pub fn yield_to_accelerator(&mut self) -> Result<()> {
        self.record_host_call(HostCall::Yield);
        self.internal_yield()
    }

    /// Sends an `ArbCmd` message to one of the plugins, referenced by name.
    ///
    /// `ArbCmd`s are executed immediately after yielding to the simulator, so
    /// all pending asynchronous calls are flushed and executed *before* the
    /// `ArbCmd`.
    pub fn arb(&mut self, name: impl AsRef<str>, cmd: impl Into<ArbCmd>) -> Result<ArbData> {
        let name = name.as_ref();
        for (i, p) in self.pipeline.iter().enumerate() {
            if p.plugin.name() == name {
                return self.arb_idx(i as isize, cmd);
            }
        }
        inv_arg(format!("plugin {} not found", name))
    }

    /// Checks plugin index range and allows using negative numbers for
    /// Pythonic indexation.
    fn convert_plugin_index(&self, index: isize) -> Result<usize> {
        let mut conv_index = index;
        let n_plugins = self.pipeline.len();
        if conv_index < 0 {
            conv_index += n_plugins as isize;
            if conv_index < 0 {
                inv_arg(format!("index {} out of range", index))?
            }
        }
        let conv_index = conv_index as usize;
        if conv_index >= n_plugins {
            inv_arg(format!("index {} out of range", index))?
        }
        Ok(conv_index)
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
        let index = self.convert_plugin_index(index)?;

        // Perform the actual call.
        let cmd = cmd.into();
        self.record_host_call(HostCall::Arb(
            self.pipeline[index].plugin.name(),
            cmd.clone(),
        ));
        self.internal_yield()?;
        self.pipeline[index].plugin.arb(cmd)
    }

    /// Returns a reference to the metadata object belonging to the plugin
    /// referenced by instance name.
    pub fn get_metadata(&self, name: impl AsRef<str>) -> Result<&PluginMetadata> {
        let name = name.as_ref();
        for (i, p) in self.pipeline.iter().enumerate() {
            if p.plugin.name() == name {
                return self.get_metadata_idx(i as isize);
            }
        }
        inv_arg(format!("plugin {} not found", name))
    }

    /// Returns a reference to the metadata object belonging to the plugin
    /// referenced by index.
    pub fn get_metadata_idx(&self, index: isize) -> Result<&PluginMetadata> {
        Ok(&self.pipeline[self.convert_plugin_index(index)?].metadata)
    }

    /// Writes a the reproduction log to a file.
    pub fn write_reproduction_file(&self, filename: impl AsRef<Path>) -> Result<()> {
        if let Some(log) = &self.reproduction_log {
            log.to_file(filename)
        } else {
            inv_op(
                "cannot output reproduction file; \
                 we failed earlier on when attempting to construct the logger.",
            )
        }
    }
}

impl Accelerator for Simulation {
    /// Starts a program on the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    fn start(&mut self, args: impl Into<ArbData>) -> Result<()> {
        if self.state.is_idle() {
            let args = args.into();
            self.record_host_call(HostCall::Start(args.clone()));
            self.state.put_data(args).unwrap();
            Ok(())
        } else {
            inv_op("accelerator is already running; call wait() first")
        }
    }

    /// Waits for the accelerator to finish its current program.
    ///
    /// When this succeeds, the return value of the accelerator's `run()`
    /// function is returned.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    fn wait(&mut self) -> Result<ArbData> {
        self.record_host_call(HostCall::Wait);
        if self.state.is_wait_pending() {
            self.state.take_data()
        } else {
            self.internal_yield()?;
            if self.state.is_wait_pending() {
                self.state.take_data()
            } else {
                err("Deadlock: accelerator is blocked on recv() while we are expecting it to return")
            }
        }
    }

    /// Sends a message to the accelerator.
    ///
    /// This is an asynchronous call: nothing happens until `yield()`,
    /// `recv()`, or `wait()` is called.
    fn send(&mut self, args: impl Into<ArbData>) -> Result<()> {
        let args = args.into();
        self.record_host_call(HostCall::Send(args.clone()));
        self.host_to_accelerator_data.push_back(args);
        Ok(())
    }

    /// Waits for the accelerator to send a message to us.
    ///
    /// Deadlocks are detected and prevented by throwing an error message.
    fn recv(&mut self) -> Result<ArbData> {
        self.record_host_call(HostCall::Recv);
        if let Some(data) = self.accelerator_to_host_data.pop_front() {
            Ok(data)
        } else {
            self.internal_yield()?;
            if let Some(data) = self.accelerator_to_host_data.pop_front() {
                Ok(data)
            } else {
                err("Deadlock: recv() called while queue is empty and accelerator is idle")
            }
        }
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        trace!("Dropping Simulation");
    }
}
