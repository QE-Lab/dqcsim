use crate::{
    common::{
        error::{inv_op, Result},
        log::Loglevel,
        protocol::{
            FrontendRunRequest, FrontendRunResponse, GatestreamUp, PluginInitializeRequest,
            PluginInitializeResponse, PluginToSimulator, SimulatorToPlugin,
        },
        types::{ArbCmd, ArbData, Gate, PluginType, QubitRef, SequenceNumber},
    },
    plugin::{
        connection::{Connection, IncomingMessage, OutgoingMessage},
        definition::PluginDefinition,
        log::setup_logging,
    },
    trace,
};

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

        // Setup logging.
        setup_logging(&req.log_configuration, req.log_channel)?;

        trace!("started handle_init()!");

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

        // Store the incoming messages for recv().
        // TODO

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
        // TODO
        let messages = vec![];

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
                let response = OutgoingMessage::Simulator(match message {
                    SimulatorToPlugin::Initialize(req) => match self.handle_init(*req) {
                        Ok(x) => PluginToSimulator::Initialized(x),
                        Err(e) => PluginToSimulator::Failure(e.to_string()),
                    },
                    SimulatorToPlugin::AcceptUpstream => match self.handle_accept_upstream() {
                        Ok(_) => PluginToSimulator::Success,
                        Err(e) => PluginToSimulator::Failure(e.to_string()),
                    },
                    SimulatorToPlugin::Abort => {
                        aborted = true;
                        match self.handle_abort() {
                            Ok(_) => PluginToSimulator::Success,
                            Err(e) => PluginToSimulator::Failure(e.to_string()),
                        }
                    }
                    SimulatorToPlugin::RunRequest(req) => match self.handle_run(req) {
                        Ok(x) => PluginToSimulator::RunResponse(x),
                        Err(e) => PluginToSimulator::Failure(e.to_string()),
                    },
                    SimulatorToPlugin::ArbRequest(_) => {
                        // TODO
                        PluginToSimulator::ArbResponse(ArbData::default())
                    }
                });
                self.connection.send(response)?;
            }
            IncomingMessage::Upstream(_) => {
                unimplemented!();
            }
            IncomingMessage::Downstream(_) => {
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
    pub fn send(&mut self, _msg: ArbData) -> Result<()> {
        unimplemented!();
    }

    /// Waits for a message from the host.
    ///
    /// It is only legal to call this function from within the `run()`
    /// callback. Any other source will result in an `Err` return value.
    pub fn recv(&mut self) -> Result<ArbData> {
        unimplemented!();
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
        unimplemented!();
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
        unimplemented!();
    }
}
