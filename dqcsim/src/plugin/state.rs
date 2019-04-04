use crate::{
    common::{
        error::{err, inv_op, oe_err, Result},
        log::Loglevel,
        protocol::{
            FrontendRunRequest, FrontendRunResponse, GatestreamUp, PluginInitializeRequest,
            PluginInitializeResponse, PluginToSimulator, SimulatorToPlugin,
        },
        types::{ArbCmd, ArbData, Gate, PluginType, QubitRef, SequenceNumber},
    },
    error,
    plugin::{
        connection::{Connection, IncomingMessage, OutgoingMessage},
        definition::PluginDefinition,
        log::setup_logging,
    },
    trace,
};
use rand::distributions::Standard;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use std::collections::VecDeque;

/// Deterministic random number generator used for plugins.
///
/// This actually contains multiple RNGs: one for each incoming message stream.
/// This is necessary because messages arrive in a deterministic order only
/// within the context of a single stream; the rest is up to the OS, thread
/// scheduling, etc.
struct RandomNumberGenerator {
    rngs: Vec<ChaChaRng>,
    selected: usize,
}

impl RandomNumberGenerator {
    /// Constructs a random number generator with the specified number of
    /// deterministic streams seeded by the specified seed.
    pub fn new(num_streams: usize, seed: u64) -> RandomNumberGenerator {
        let mut rng = ChaChaRng::seed_from_u64(seed);
        let mut rngs = vec![];
        for _ in 1..num_streams {
            rngs.push(ChaChaRng::seed_from_u64(rng.next_u64()));
        }
        rngs.push(rng);
        RandomNumberGenerator { rngs, selected: 0 }
    }

    /// Selects the current RNG.
    pub fn select(&mut self, index: usize) {
        assert!(index < self.rngs.len());
        self.selected = index;
    }

    /// Generates a random 64-bit number using the active RNG.
    pub fn random_u64(&mut self) -> u64 {
        self.rngs[self.selected].next_u64()
    }

    /// Generates a random floating point number in the range `[0,1>` using the
    /// active RNG.
    pub fn random_f64(&mut self) -> f64 {
        self.rngs[self.selected].sample(Standard)
    }
}

/// Structure representing the state of a plugin.
///
/// This contains all state and connection information. The public members are
/// exposed as user API calls.
#[allow(dead_code)] // TODO <-- remove me
pub struct PluginState<'a> {
    /// PluginDefinition structure containing the callback closures and some
    /// metadata. This must never change during the execution of the plugin.
    definition: &'a PluginDefinition,

    /// Connection object, representing the connections to the host/simulator,
    /// the upstream plugin (if any), and the downstream plugin (if any).
    connection: Connection,

    /// Set when we're a frontend and we're inside a run() callback.
    inside_run: bool,

    /// Objects queued with `send()`, to be sent to the host in the next
    /// RunResponse.
    frontend_to_host_data: VecDeque<ArbData>,

    /// Objects received from the host, to be consumed using `recv()`.
    host_to_frontend_data: VecDeque<ArbData>,

    /// Random number generator.
    rng: Option<RandomNumberGenerator>,

    // TODO: internal state such as cached measurement, sequence number
    // counters, etc.
}

#[allow(dead_code)] // TODO <-- remove me
impl<'a> PluginState<'a> {
    /// Blockingly receive messages from downstream until all requests have
    /// been acknowledged.
    fn synchronize_downstream(&mut self) -> Result<()> {
        /*while ... {
            self.connextion.next_response(...)
            ...
            self.handle_downstream(...)
        }*/
        // TODO
        Ok(())
    }

    /// Blockingly receive messages from downstream until the request with the
    /// specified sequence number has been acknowledged.
    fn synchronize_downstream_up_to(&mut self, _num: SequenceNumber) -> Result<()> {
        /*while ... {
            self.connextion.next_response(...)
            ...
            self.handle_downstream(...)
        }*/
        // TODO
        Ok(())
    }

    /// Handle an incoming downstream message.
    fn handle_downstream(&mut self, _msg: GatestreamUp) -> Result<()> {
        // TODO: update our cached measurement, simulation timing etc states
        Ok(())
    }

    /// Handles a SimulatorToPlugin::Initialize RPC.
    fn handle_init(&mut self, req: PluginInitializeRequest) -> Result<(PluginInitializeResponse)> {
        let typ = self.definition.get_type();
        let seed = req.seed;

        // Setup logging.
        setup_logging(&req.log_configuration, req.log_channel)?;

        trace!("started handle_init()!");

        // Seed RNGs.
        trace!("seeding with value {}", seed);
        self.rng.replace(RandomNumberGenerator::new(3, seed));

        // Make sure that we're the type of plugin that the simulator is
        // expecting.
        if typ != req.plugin_type {
            inv_op(format!(
                "host is expecting a plugin of type {:?}, but we're a plugin of type {:?}",
                req.plugin_type, typ
            ))?;
        }

        // Connect to downstream plugin, unless we're a backend.
        if typ != PluginType::Backend {
            self.connection
                .connect_downstream(req.downstream.unwrap())?;
        }

        // Run the initialize callback.
        (self.definition.initialize)(self, req.init_cmds)?;

        // If we're not a frontend, initialize an upstream server.
        let upstream = if typ == PluginType::Frontend {
            None
        } else {
            Some(self.connection.serve_upstream()?)
        };

        trace!("finished handle_init()!");

        Ok(PluginInitializeResponse {
            upstream,
            metadata: self.definition.get_metadata().clone(),
        })
    }

    /// Handles a SimulatorToPlugin::AcceptUpstream RPC.
    fn handle_accept_upstream(&mut self) -> Result<()> {
        trace!("started accept_upstream()!");
        let result = self.connection.accept_upstream();
        trace!("finished accept_upstream()!");
        result
    }

    /// Handles a SimulatorToPlugin::Abort RPC.
    fn handle_abort(&mut self) -> Result<()> {
        trace!("started handle_abort()!");

        // Make sure we receive gatestream acknowledgements for every request
        // we sent, ensuring that errors are propagated.
        self.synchronize_downstream()?;

        // Call the user's finalize function.
        (self.definition.drop)(self)?;

        // Finalization should not send any more requests downstream, but just
        // in case:
        self.synchronize_downstream()?;

        trace!("finished handle_abort()!");
        Ok(())
    }

    /// Handles a run request while we're NOT blocked inside the run()
    /// callback.
    fn handle_run(&mut self, req: FrontendRunRequest) -> Result<FrontendRunResponse> {
        // If we're inside a run, some internal logic did something wrong;
        // FrontendRunRequests must be handled differently in this case.
        assert!(
            !self.inside_run,
            "handle_run() can only be used outside of the run() callback"
        );
        if self.definition.get_type() != PluginType::Frontend {
            inv_op("received run request from simulator, but we're not a frontend!")?;
        }

        // Store the incoming messages for recv().
        self.host_to_frontend_data.extend(req.messages);

        // If start is set, call the run() callback.
        let return_value = if let Some(args) = req.start {
            self.inside_run = true;
            let return_value = (self.definition.run)(self, args)?;
            self.inside_run = false;
            Some(return_value)
        } else {
            None
        };

        // Drain the messages queued up by send().
        let messages = self.frontend_to_host_data.drain(..).collect();

        Ok(FrontendRunResponse {
            return_value,
            messages,
        })
    }

    /// Handles any incoming message.
    ///
    /// The returned boolean indicates whether the message was an abort,
    /// implying that we should break out of our run loop.
    fn handle_incoming_message(&mut self, request: IncomingMessage) -> Result<bool> {
        let mut aborted = false;
        match request {
            IncomingMessage::Simulator(message) => {
                if let Some(ref mut rng) = self.rng {
                    rng.select(0)
                }
                let response = OutgoingMessage::Simulator(match message {
                    SimulatorToPlugin::Initialize(req) => match self.handle_init(*req) {
                        Ok(x) => PluginToSimulator::Initialized(x),
                        Err(e) => {
                            let e = e.to_string();
                            error!("{}", e);
                            PluginToSimulator::Failure(e.to_string())
                        }
                    },
                    SimulatorToPlugin::AcceptUpstream => match self.handle_accept_upstream() {
                        Ok(_) => PluginToSimulator::Success,
                        Err(e) => {
                            let e = e.to_string();
                            error!("{}", e);
                            PluginToSimulator::Failure(e.to_string())
                        }
                    },
                    SimulatorToPlugin::Abort => {
                        aborted = true;
                        match self.handle_abort() {
                            Ok(_) => PluginToSimulator::Success,
                            Err(e) => {
                                let e = e.to_string();
                                error!("{}", e);
                                PluginToSimulator::Failure(e.to_string())
                            }
                        }
                    }
                    SimulatorToPlugin::RunRequest(req) => match self.handle_run(req) {
                        Ok(x) => PluginToSimulator::RunResponse(x),
                        Err(e) => {
                            let e = e.to_string();
                            error!("{}", e);
                            PluginToSimulator::Failure(e.to_string())
                        }
                    },
                    SimulatorToPlugin::ArbRequest(req) => {
                        match (self.definition.host_arb)(self, req) {
                            Ok(x) => PluginToSimulator::ArbResponse(x),
                            Err(e) => {
                                let e = e.to_string();
                                error!("{}", e);
                                PluginToSimulator::Failure(e.to_string())
                            }
                        }
                    }
                });
                self.connection.send(response)?;
            }
            IncomingMessage::Upstream(_) => {
                if let Some(ref mut rng) = self.rng {
                    rng.select(1)
                }
                unimplemented!();
            }
            IncomingMessage::Downstream(_) => {
                if let Some(ref mut rng) = self.rng {
                    rng.select(2)
                }
                unimplemented!();
            }
        }
        Ok(aborted)
    }

    // Note that the functions above are intentionally NOT public. Only
    // PluginState and we ourselves need to access it, and they're in the
    // same module so this is allowed. Also tests of course, which are in a
    // child module, which also makes it legal. The functions below this point
    // are API calls for the user logic.

    /// Executes a plugin described by `definition` within the context of the
    /// specified simulator endpoint address.
    pub fn run(definition: &'a PluginDefinition, simulator: impl Into<String>) -> Result<()> {
        let mut state = PluginState {
            definition,
            connection: Connection::new(simulator)?,
            inside_run: false,
            frontend_to_host_data: VecDeque::new(),
            host_to_frontend_data: VecDeque::new(),
            rng: None,
        };

        while let Some(request) = state.connection.next_request()? {
            if state.handle_incoming_message(request)? {
                break;
            }
        }
        Ok(())
    }

    /// Sends a log message to DQCsim by means of a LogRecord structure.
    pub fn log<T, S>(
        &mut self,
        _message: impl Into<String>,
        _level: impl Into<Loglevel>,
        _module: Option<T>,
        _file: Option<S>,
        _line_nr: impl Into<Option<u32>>,
    ) where
        T: Into<String>,
        S: Into<String>,
    {
        unimplemented!();
    }

    /// Sends a message to the host.
    ///
    /// It is only legal to call this function from within the `run()`
    /// callback. Any other source will result in an `Err` return value.
    pub fn send(&mut self, msg: ArbData) -> Result<()> {
        if !self.inside_run {
            inv_op("send() can only be called from inside the run() callback")?;
        }
        self.frontend_to_host_data.push_back(msg);
        Ok(())
    }

    /// Waits for a message from the host.
    ///
    /// It is only legal to call this function from within the `run()`
    /// callback. Any other source will result in an `Err` return value.
    pub fn recv(&mut self) -> Result<ArbData> {
        if !self.inside_run {
            inv_op("recv() can only be called from inside the run() callback")?;
        }
        while self.host_to_frontend_data.is_empty() {
            // We need to yield to the host! Send the RunResponse message now.
            // Don't forget to drain the messages queued up by send().
            self.connection
                .send(OutgoingMessage::Simulator(PluginToSimulator::RunResponse(
                    FrontendRunResponse {
                        return_value: None,
                        messages: self.frontend_to_host_data.drain(..).collect(),
                    },
                )))
                .unwrap();

            // Fetch the next message.
            let request = self
                .connection
                .next_request()?
                .ok_or_else(oe_err("Simulation aborted"))?;

            // If the message is a RunRequest, we need to handle it locally.
            // All other messages are handled the usual way using
            // `handle_incoming_message()`.
            if let IncomingMessage::Simulator(SimulatorToPlugin::RunRequest(request)) = request {
                // Make sure to select the right RNG.
                if let Some(ref mut rng) = self.rng {
                    rng.select(0)
                }

                // Store the incoming messages for recv().
                self.host_to_frontend_data.extend(request.messages);

                // If start is set, call the run() callback.
                if request.start.is_some() {
                    return err("Protocol error: cannot start accelerator while accelerator is already running");
                }
                continue;
            } else if self.handle_incoming_message(request)? {
                return err("Simulation aborted");
            }
        }
        Ok(self.host_to_frontend_data.pop_front().unwrap())
    }

    /// Allocate the given number of downstream qubits.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn allocate(&mut self, _num_qubits: usize, _arbs: Vec<ArbCmd>) -> Result<Vec<QubitRef>> {
        unimplemented!();
    }

    /// Free the given downstream qubits.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn free(&mut self, _qubits: Vec<QubitRef>) -> Result<()> {
        unimplemented!();
    }

    /// Tells the downstream plugin to execute a gate.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn gate(&mut self, _gate: Gate) -> Result<()> {
        unimplemented!();
    }

    /// Returns the latest measurement of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_measurement(&self, _qubit: QubitRef) -> Result<bool> {
        unimplemented!();
    }

    /// Returns the additional data associated with the latest measurement of
    /// the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_measurement_arb(&self, _qubit: QubitRef) -> Result<ArbData> {
        unimplemented!();
    }

    /// Returns the number of downstream cycles since the latest measurement
    /// of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycles_since_measure(&self, _qubit: QubitRef) -> Result<u64> {
        unimplemented!();
    }

    /// Returns the number of downstream cycles between the last two
    /// measurements of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycles_between_measures(&self, _qubit: QubitRef) -> Result<u64> {
        unimplemented!();
    }

    /// Advances the downstream cycle counter.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn advance(&mut self, _cycles: u64) -> Result<u64> {
        unimplemented!();
    }

    /// Returns the current value of the downstream cycle counter.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycle(&self) -> Result<u64> {
        unimplemented!();
    }

    /// Sends an arbitrary command downstream.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn arb(&mut self, _cmd: ArbCmd) -> Result<ArbData> {
        unimplemented!();
    }

    /// Generates a random unsigned 64-bit number using the simulator random
    /// seed.
    ///
    /// This function may use one of two pseudorandom number generator states
    /// depending on whether it is called synchronized to the downstream
    /// command flow or synchronized to the upstream measurement flow (i.e.
    /// called by `modify_measurements()`). This is to ensure that the order in
    /// which the RNG functions are called per state does not depend on OS
    /// scheduling.
    pub fn random_u64(&mut self) -> u64 {
        self.rng.as_mut().expect("RNG not initialized").random_u64()
    }

    /// Generates a random floating point number using the simulator random
    /// seed.
    ///
    /// The generated numbers are in the range `[0,1>`.
    ///
    /// This function may use one of two pseudorandom number generator states
    /// depending on whether it is called synchronized to the downstream
    /// command flow or synchronized to the upstream measurement flow (i.e.
    /// called by `modify_measurements()`). This is to ensure that the order in
    /// which the RNG functions are called per state does not depend on OS
    /// scheduling.
    pub fn random_f64(&mut self) -> f64 {
        self.rng.as_mut().expect("RNG not initialized").random_f64()
    }
}
