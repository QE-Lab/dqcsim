//! Module containing the runtime structures of a plugin.

use crate::{
    common::{
        error::{err, inv_arg, inv_op, oe_err, Result},
        protocol::{
            FrontendRunRequest, FrontendRunResponse, GatestreamDown, GatestreamUp,
            PipelinedGatestreamDown, PluginInitializeRequest, PluginInitializeResponse,
            PluginToSimulator, SimulatorToPlugin,
        },
        types::{
            ArbCmd, ArbData, Cycle, Cycles, Gate, PluginType, QubitMeasurementResult,
            QubitMeasurementValue, QubitRef, QubitRefGenerator, SequenceNumber,
            SequenceNumberGenerator,
        },
        util::friendly_enumerate,
    },
    debug, error, fatal,
    plugin::{
        connection::{Connection, IncomingMessage, OutgoingMessage},
        definition::PluginDefinition,
        log::setup_logging,
    },
    trace, warn,
};
use rand::distributions::Standard;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use std::collections::{HashMap, HashSet, VecDeque};

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

    /// Returns the currently selected RNG.
    pub fn get_selected(&self) -> usize {
        self.selected
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

/// Structure containing all the classical data associated with a qubit
/// measurement.
#[derive(Debug, Clone)]
struct QubitMeasurementData {
    /// The value of the latest measurement for this qubit.
    value: QubitMeasurementValue,

    /// The data attached to the latest measurement of this qubit.
    data: ArbData,

    /// The timestamp of the latest measurement.
    timestamp: Cycle,

    /// The time that the qubit had gone without measurement at the time of the
    /// latest measurement, if this is not the first measurement.
    timer: Option<Cycles>,
}

/// Structure containing the data we need to keep track of for each qubit.
#[derive(Debug, Clone)]
struct QubitData {
    /// The latest measurement data for this qubit.
    measurement: Option<QubitMeasurementData>,

    /// The downstream sequence number of the gate that last affected this
    /// qubit. Before using the enclosed measurement data, the simulation must
    /// always be synchronized up to this point.
    last_mutation: SequenceNumber,
}

/// Structure representing the state of a plugin.
///
/// This contains all state and connection information. The public members are
/// exposed as user API calls.
pub struct PluginState<'a> {
    /// PluginDefinition structure containing the callback closures and some
    /// metadata. This must never change during the execution of the plugin.
    definition: &'a PluginDefinition,

    /// Connection object, representing the connections to the host/simulator,
    /// the upstream plugin (if any), and the downstream plugin (if any).
    connection: Connection,

    /// Set when we're a frontend and we're inside a run() callback.
    inside_run: bool,

    /// True when the callbacks we're executing are synchronous to an RPC.
    /// False when they're synchronous to the gatestream responses.
    synchronized_to_rpcs: bool,

    /// Objects queued with `send()`, to be sent to the host in the next
    /// RunResponse.
    frontend_to_host_data: VecDeque<ArbData>,

    /// Objects received from the host, to be consumed using `recv()`.
    host_to_frontend_data: VecDeque<ArbData>,

    /// Random number generator.
    rng: Option<RandomNumberGenerator>,

    /// Upstream qubit reference generator.
    ///
    /// This is used to allocate/free qubit references when we receive a
    /// message from upstream. This generator should always be in sync with
    /// downstream_qubit_ref_generator of the upstream plugin.
    upstream_qubit_ref_generator: QubitRefGenerator,

    /// The upstream sequence number up to which we've called the user's
    /// callbacks for.
    ///
    /// This is NOT necessarily the point up to which we've actually completed
    /// the requests from the upstream plugin's perspective if we're an
    /// operator. Specifically operator.gate() calls may postpone delivery of
    /// measurement results through the operator.modify_measurement() callback.
    /// We can only send the CompletedUpTo message when all the qubit results
    /// for these messages are in.
    upstream_issued_up_to: SequenceNumber,

    /// Stores mappings from downstream sequence numbers (.0) to upstream
    /// sequence numbers (.1) to memorize postponed measurement results in
    /// operator plugins.
    ///
    /// When a measurement gate is performed that does not immediately return
    /// all measurement results, an entry is made here using the current
    /// downstream TX sequence number and the current upstream sequence number.
    /// We're only allowed to acknowledge the upstream sequence number with
    /// `CompletedUpTo` when the downstream sequence number is acknowledged.
    /// At that point the entry is removed from this deque.
    upstream_postponed: VecDeque<(SequenceNumber, SequenceNumber)>,

    /// The latest upstream sequence number for which we've sent the
    /// `CompletedUpTo` message.
    upstream_completed_up_to: SequenceNumber,

    /// Downstream sequence number generator.
    downstream_sequence_tx: SequenceNumberGenerator,

    /// Latest acknowledged downstream sequence number (= number of requests
    /// acknowledged).
    downstream_sequence_rx: SequenceNumber,

    /// Downstream simulation time, TX-synchronized.
    downstream_cycle_tx: Cycle,

    /// Downstream simulation time, RX-synchronized.
    downstream_cycle_rx: Cycle,

    /// Downstream qubit reference generator.
    ///
    /// This is used to allocate/free qubit references when the user tells us
    /// to do this in the downstream domain.  This generator should always be
    /// in sync with upstream_qubit_ref_generator of the downstream plugin.
    downstream_qubit_ref_generator: QubitRefGenerator,

    /// Current state of the qubit measurement bits. The keys in this map also
    /// function as the set of all live downstream qubit references.
    downstream_qubit_data: HashMap<QubitRef, QubitData>,

    /// Measurement results from downstream, queued until we get the sequence
    /// number they belong to.
    downstream_measurement_queue: VecDeque<QubitMeasurementResult>,

    /// Expected measurements according to the `measures` sets of the gates
    /// we sent downstream.
    downstream_expected_measurements: VecDeque<(SequenceNumber, HashSet<QubitRef>)>,

    /// Aborted flag indicates if the plugin received the aborted signal.
    aborted: bool,
}

impl<'a> PluginState<'a> {
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
            let return_value = (self.definition.run)(self, args);
            self.inside_run = false;
            Some(return_value?)
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

    /// Processes a checked measurement.
    ///
    /// That is, saves it to our cache, and forwards it upstream if we're an
    /// operator.
    fn handle_measurement(&mut self, measurement: QubitMeasurementResult) -> Result<()> {
        // Note that it's not an error if there is no data entry for the
        // received qubit (anymore). This just means that the qubit has been
        // freed soon after it was measured, the measurement result was never
        // read/waited upon, and the downstream plugin is sufficiently lagging
        // behind us.
        if let Some(data) = self.downstream_qubit_data.get_mut(&measurement.qubit) {
            trace!("Caching measurement for qubit {}...", measurement.qubit);

            // Current simulation time.
            let timestamp = self.downstream_cycle_rx;

            // Cycles between the previous measurement and the current
            // simulation time.
            let timer = if let Some(x) = &data.measurement {
                let delta = timestamp - x.timestamp;
                if delta < 0 {
                    panic!("simulation time is apparently not monotonous?");
                }
                Some(delta as Cycles)
            } else {
                None
            };

            // Update the measurement data.
            data.measurement.replace(QubitMeasurementData {
                value: measurement.value,
                data: measurement.data.clone(),
                timestamp,
                timer,
            });

            // If we're an operator, propagate the measurement upstream using
            // the `modify_measurement()` callback.
            if self.definition.get_type() == PluginType::Operator {
                let measurements = (self.definition.modify_measurement)(self, measurement)?;
                for measurement in measurements {
                    self.connection
                        .send(OutgoingMessage::Upstream(GatestreamUp::Measured(
                            measurement,
                        )))?;
                }
            }
        } else {
            trace!(
                "Not caching measurement for qubit {}; no data exists (anymore)",
                measurement.qubit
            );
        }
        Ok(())
    }

    /// Verifies that the queued measurement results correspond with the
    /// `measures` vectors in the respective gates that we sent, and saves the
    /// downstream sequence number. We may also need to forward the
    /// `CompletedUpTo` message upstream (with the sequence number mapped to
    /// upstream sequence numbers appropriately) if previously postponed
    /// results have been received.
    fn received_downstream_sequence(&mut self, sequence: SequenceNumber) -> Result<()> {
        trace!("Downstream completed up to {}", sequence);
        // Update the sequence number.
        self.downstream_sequence_rx = sequence;

        // Check queued measurements against the ones expected from the gates
        // that we sent.
        let measurements: Vec<_> = self.downstream_measurement_queue.drain(..).collect();
        for measurement in measurements {
            // pop is set when we've received all the measurements for the
            // current gate (at the front of the downstream_expected_measurements
            // queue), i.e. the HashSet containing the qubits is empty. If this
            // is the case we should pop the gate off of the expected measurement
            // queue.
            let mut pop = false;

            // ok is set if the measurement was part of the current gate's
            // measures set. If ok is set, the qubit has already been removed
            // from the gate's (remaining) expected measurement set, but the
            // measurement has not been handled yet.
            let mut ok = false;

            // Note that we're using the above two flags to keep Ferris happy;
            // we can't use self within the if let due to the mutable borrow.
            if let Some(expected) = self.downstream_expected_measurements.front_mut() {
                if sequence.acknowledges(expected.0) && expected.1.remove(&measurement.qubit) {
                    ok = true;
                    pop = expected.1.is_empty();
                }
            }

            // Do what we just determined we need to do.
            if ok {
                // Handle the received measurement.
                self.handle_measurement(measurement)?;

                // Clean up/move on to the next gate if we received everything
                // we were expecting for the current gate.
                if pop {
                    self.downstream_expected_measurements.pop_front().unwrap();
                }
            } else {
                // Unexpected measurement. We always IGNORE these. This gives
                // consistent, deterministic behavior of the measurement cache
                // as long as the measurements are received in the same order
                // every time.
                warn!(
                    "ignored unexpected measurement data for qubit {}; bug in downstream plugin!",
                    measurement.qubit
                );
            }
        }

        // Check that we received all the measurements we were expecting to
        // receive thus far. Equivalently, stuff is wrong if the sequence
        // number we just received acknowledges more of the gates still
        // remaining in the queue.
        loop {
            // pop is set when the current gate (at the front of the
            // downstream_expected_measurements queue) is acknowledged by the
            // received sequence number.
            let mut pop = false;

            // Note that we're using that flag to keep Ferris happy; we can't
            // use self within the if let due to the mutable borrow.
            if let Some(expected) = self.downstream_expected_measurements.front_mut() {
                if sequence.acknowledges(expected.0) {
                    pop = true;
                }
            }

            // If the current gate was not acknowledged by this sequence
            // number, everything's synchronized.
            if !pop {
                break;
            }

            // Uh oh, the current gate was (still) expecting measurements.
            // So we just fabricate some measurements (with the value set to
            // "undefined" of course) to work around the downstream plugin's
            // bugs. We also move on to the next gate (note the pop_front in
            // the iterator).
            for qubit in self
                .downstream_expected_measurements
                .pop_front()
                .unwrap()
                .1
                .drain()
            {
                if self.downstream_qubit_data.contains_key(&qubit) {
                    warn!(
                        "missing measurement data for qubit {}, setting to undefined; bug in downstream plugin!",
                        qubit
                    );
                    self.handle_measurement(QubitMeasurementResult::new(
                        qubit,
                        QubitMeasurementValue::Undefined,
                        ArbData::default(),
                    ))?;
                } else {
                    trace!(
                        "missing measurement data for qubit {}, which has already been deallocated",
                        qubit
                    );
                }
            }
        }

        // Update the upstream_postponed mapping to see if we can propagate the
        // acknowledgement upstream.
        let mut updates = false;
        while !self.upstream_postponed.is_empty() {
            let mut acknowledged = false;
            if let Some((downstream, _)) = self.upstream_postponed.front() {
                acknowledged = self.downstream_sequence_rx.acknowledges(*downstream);
            }
            if acknowledged {
                updates = true;
                self.upstream_postponed.pop_front();
            } else {
                break;
            }
        }
        if updates {
            self.check_completed_up_to()?;
        }

        Ok(())
    }

    /// Check whether we can/need to send the next CompletedUpTo message.
    fn check_completed_up_to(&mut self) -> Result<()> {
        let mut completed_up_to = self.upstream_issued_up_to;

        // Check the upstream_postponed map to see if any command with an
        // upstream sequence number lower than self.upstream_issued_up_to
        // still has postponed results that have not been received from
        // downstream yet. In that case, we can only indicate completion up to
        // the sequence number immediately before that one.
        if let Some((_, upstream)) = self.upstream_postponed.front() {
            if completed_up_to.after(upstream.preceding()) {
                completed_up_to = upstream.preceding();
            }
        }

        // Send a `CompletedUpTo` message if there are any requests to
        // acknowledge.
        if completed_up_to.after(self.upstream_completed_up_to) {
            trace!("We've completed up to {}", completed_up_to);
            self.connection
                .send(OutgoingMessage::Upstream(GatestreamUp::CompletedUpTo(
                    completed_up_to,
                )))?;
            self.upstream_completed_up_to = completed_up_to;
        }
        Ok(())
    }

    /// Handle an incoming upstream message from the downstream plugin.
    fn handle_downstream_message(&mut self, message: GatestreamUp) -> Result<()> {
        if let Some(ref mut rng) = self.rng {
            rng.select(2);
        }
        self.synchronized_to_rpcs = false;

        match message {
            GatestreamUp::CompletedUpTo(sequence) => {
                self.received_downstream_sequence(sequence)?;
            }
            GatestreamUp::Failure(sequence, message) => {
                error!("Error from downstream plugin: {}", message);
                debug!("The sequence number was {}", sequence);
                fatal!("Desynchronized with downstream plugin due to downstream error, cannot continue!");
                err(format!(
                    "simulation failed due to downstream error: {}",
                    message
                ))?;
            }
            GatestreamUp::Measured(measurement) => {
                trace!(
                    "Downstream sent measurement for qubit {}",
                    measurement.qubit
                );
                self.downstream_measurement_queue.push_back(measurement);
            }
            GatestreamUp::Advanced(cycles) => {
                self.downstream_cycle_rx = self.downstream_cycle_rx.advance(cycles);
            }
            x => {
                error!("Unexpected message received from downstream");
                trace!("{:?}", x);
                err("unexpected message received from downstream")?;
            }
        }
        Ok(())
    }

    /// Handles any incoming message.
    ///
    /// The returned boolean indicates whether the message was an abort,
    /// implying that we should break out of our run loop.
    fn handle_incoming_message(&mut self, request: IncomingMessage) -> Result<bool> {
        // Don't handle message after Abort request has been handled.
        if !self.aborted {
            match request {
                IncomingMessage::Simulator(message) => {
                    if let Some(ref mut rng) = self.rng {
                        rng.select(0)
                    }
                    self.synchronized_to_rpcs = true;

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
                        SimulatorToPlugin::UserInitialize(req) => {
                            match (self.definition.initialize)(self, req.init_cmds) {
                                Ok(_) => PluginToSimulator::Success,
                                Err(e) => {
                                    let e = e.to_string();
                                    error!("{}", e);
                                    PluginToSimulator::Failure(e.to_string())
                                }
                            }
                        }
                        SimulatorToPlugin::Abort => {
                            self.aborted = true;
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

                    // Before we return control back to the host, make sure
                    // that the gatestream is synchronized (#90). If we don't
                    // do this, ArbCmds sent to downstream plugins by the host
                    // may not be properly synchronized.
                    self.synchronize_downstream()?;

                    self.connection.send(response)?;
                }
                IncomingMessage::Upstream(GatestreamDown::Pipelined(sequence, message)) => {
                    if let Some(ref mut rng) = self.rng {
                        rng.select(1)
                    }
                    self.synchronized_to_rpcs = true;

                    let response = match message {
                        PipelinedGatestreamDown::Allocate(num_qubits, commands) => {
                            let qubits = self.upstream_qubit_ref_generator.allocate(num_qubits);
                            (self.definition.allocate)(self, qubits, commands)
                        }
                        PipelinedGatestreamDown::Free(qubits) => {
                            self.upstream_qubit_ref_generator.free(qubits.clone());
                            (self.definition.free)(self, qubits)
                        }
                        PipelinedGatestreamDown::Gate(gate) => {
                            let mut measures: HashSet<_> =
                                gate.get_measures().iter().cloned().collect();
                            (self.definition.gate)(self, gate).and_then(|measurements| {
                            for measurement in measurements {
                                if measures.remove(&measurement.qubit) {
                                    self.connection
                                        .send(OutgoingMessage::Upstream(GatestreamUp::Measured(measurement)))?;
                                } else {
                                    err(format!(
                                        "user-defined gate() function returned multiple measurements for qubit {}",
                                        measurement.qubit
                                    ))?;
                                }
                            }
                            if !measures.is_empty() {
                                if self.definition.get_type() == PluginType::Operator {
                                    // These measurement results are postponed
                                    // until we receive (and maybe modify) them
                                    // from downstream.
                                    trace!("Postponing measurement results for {} until downstream {}", sequence, self.downstream_sequence_tx.get_previous());
                                    self.upstream_postponed.push_back((self.downstream_sequence_tx.get_previous(), sequence));
                                } else {
                                    // Backends cannot postpone.
                                    err(format!(
                                        "user-defined gate() function failed to return measurement for qubits {}",
                                        friendly_enumerate(measures.into_iter(), Some("or"))
                                    ))?;
                                }
                            }
                            Ok(())
                        })
                        }
                        PipelinedGatestreamDown::Advance(cycles) => self
                            .connection
                            .send(OutgoingMessage::Upstream(GatestreamUp::Advanced(cycles)))
                            .and_then(|_| (self.definition.advance)(self, cycles)),
                    };

                    // Propagate errors.
                    if let Err(e) = response {
                        let e = e.to_string();
                        error!("{}", e);
                        self.connection
                            .send(OutgoingMessage::Upstream(GatestreamUp::Failure(
                                sequence, e,
                            )))?;
                    }

                    // Save that we've completed the downstream handling of the
                    // upstream requests stream up to this point.
                    self.upstream_issued_up_to = sequence;
                    trace!("We've just finished issuing {}", sequence);

                    // Changing upstream_issued_up_to means we may have to send
                    // the next CompletedUpTo message (probably, actually).
                    self.check_completed_up_to()?;
                }
                IncomingMessage::Upstream(GatestreamDown::ArbRequest(cmd)) => {
                    if let Some(ref mut rng) = self.rng {
                        rng.select(1)
                    }
                    self.synchronized_to_rpcs = true;

                    let response = match (self.definition.upstream_arb)(self, cmd) {
                        Ok(r) => GatestreamUp::ArbSuccess(r),
                        Err(e) => GatestreamUp::ArbFailure(e.to_string()),
                    };
                    self.connection.send(OutgoingMessage::Upstream(response))?;
                }
                IncomingMessage::Downstream(message) => self.handle_downstream_message(message)?,
            }
        }

        Ok(self.aborted)
    }

    /// Helper function for synchronize_downstream_up_to(). Do not call this
    /// directly.
    fn _synchronize_downstream_up_to(&mut self, num: SequenceNumber) -> Result<()> {
        while num.after(self.downstream_sequence_rx) {
            match self.connection.next_downstream_request()? {
                Some(IncomingMessage::Downstream(message)) => {
                    self.handle_downstream_message(message)?
                }
                Some(_) => panic!("next_downstream_request() returned a non-downstream message"),
                None => err("Simulation aborted")?,
            }
        }
        Ok(())
    }

    /// Blockingly receive messages from downstream until the request with the
    /// specified sequence number has been acknowledged.
    fn synchronize_downstream_up_to(&mut self, num: SequenceNumber) -> Result<()> {
        // While handling downstream messages, we need to select the downstream
        // PRNG and indicate that we're not synchronized to the RPCs, because
        // it's not deterministic how many downstream messages will end up
        // being handled here. This is done automatically when the downstream
        // message is handled. However, when we return, we need to restore the
        // previous state, as we're synchronous again at that point.
        let rng_index = self
            .rng
            .as_ref()
            .map(RandomNumberGenerator::get_selected)
            .unwrap_or(0);
        let result = self._synchronize_downstream_up_to(num);
        if let Some(ref mut rng) = self.rng {
            rng.select(rng_index);
        }
        self.synchronized_to_rpcs = true;
        result
    }

    /// Blockingly receive messages from downstream until all requests have
    /// been acknowledged.
    fn synchronize_downstream(&mut self) -> Result<()> {
        self.synchronize_downstream_up_to(self.downstream_sequence_tx.get_previous())
    }

    /// Checks that the qubit references in the specified iterator are all
    /// currently valid.
    fn check_qubits_live<'b, 'c>(
        &'b self,
        qubits: impl IntoIterator<Item = &'c QubitRef>,
    ) -> Result<()> {
        for qubit in qubits {
            if !self.downstream_qubit_data.contains_key(qubit) {
                inv_arg(format!("qubit {} is not allocated", qubit))?;
            }
        }
        Ok(())
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
            synchronized_to_rpcs: true,
            frontend_to_host_data: VecDeque::new(),
            host_to_frontend_data: VecDeque::new(),
            rng: None,
            downstream_qubit_ref_generator: QubitRefGenerator::new(),
            downstream_sequence_tx: SequenceNumberGenerator::new(),
            downstream_sequence_rx: SequenceNumber::none(),
            downstream_cycle_tx: Cycle::t_zero(),
            downstream_cycle_rx: Cycle::t_zero(),
            upstream_qubit_ref_generator: QubitRefGenerator::new(),
            upstream_issued_up_to: SequenceNumber::none(),
            upstream_postponed: VecDeque::new(),
            upstream_completed_up_to: SequenceNumber::none(),
            downstream_qubit_data: HashMap::new(),
            downstream_measurement_queue: VecDeque::new(),
            downstream_expected_measurements: VecDeque::new(),
            aborted: false,
        };

        while let Some(request) = state.connection.next_request()? {
            if state.handle_incoming_message(request)? {
                break;
            }
        }
        Ok(())
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
            // We need to yield to the host! Before we do that though, make
            // sure that the gatestream is synchronized (#90). If we don't,
            // ArbCmds sent to downstream plugins by the host may not be
            // properly synchronized.
            self.synchronize_downstream()?;

            // Send the RunResponse message now. Don't forget to drain the
            // messages queued up by send().
            self.connection
                .send(OutgoingMessage::Simulator(PluginToSimulator::RunResponse(
                    FrontendRunResponse {
                        return_value: None,
                        messages: self.frontend_to_host_data.drain(..).collect(),
                    },
                )))
                .unwrap();

            // Inner message loop for non-RunRequest messages. RunRequest
            // messages break out of it so the response is sent by the above
            // code.
            while self.host_to_frontend_data.is_empty() {
                // Fetch the next message.
                let request = self
                    .connection
                    .next_request()?
                    .ok_or_else(oe_err("Simulation aborted"))?;

                // If the message is a RunRequest, we need to handle it
                // locally. All other messages are handled the usual way using
                // `handle_incoming_message()`.
                if let IncomingMessage::Simulator(SimulatorToPlugin::RunRequest(request)) = request
                {
                    // Make sure to select the right RNG.
                    if let Some(ref mut rng) = self.rng {
                        rng.select(0)
                    }
                    self.synchronized_to_rpcs = true;

                    // Store the incoming messages for recv().
                    self.host_to_frontend_data.extend(request.messages);

                    // start should not be set; can't run multiple programs in
                    // parallel.
                    if request.start.is_some() {
                        return err("Protocol error: cannot start accelerator while accelerator is already running");
                    }

                    // Break out of the inner loop so the RunResponse will be
                    // sent.
                    break;
                } else if self.handle_incoming_message(request)? {
                    return err("Simulation aborted");
                }
            }
        }
        Ok(self.host_to_frontend_data.pop_front().unwrap())
    }

    /// Allocate the given number of downstream qubits.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn allocate(&mut self, num_qubits: usize, commands: Vec<ArbCmd>) -> Result<Vec<QubitRef>> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("allocate() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("allocate() cannot be called while handling a gatestream response")?;
        }

        // Allocate qubit references for the new qubits.
        let qubits = self.downstream_qubit_ref_generator.allocate(num_qubits);

        // Allocate classical storage for the new qubits.
        for qubit in qubits.iter().cloned() {
            self.downstream_qubit_data.insert(
                qubit,
                QubitData {
                    measurement: None,
                    last_mutation: SequenceNumber::none(),
                },
            );
        }

        // Send the allocate message.
        self.connection
            .send(OutgoingMessage::Downstream(GatestreamDown::Pipelined(
                self.downstream_sequence_tx.get_next(),
                PipelinedGatestreamDown::Allocate(num_qubits, commands),
            )))?;

        // Return the references to the qubits.
        Ok(qubits)
    }

    /// Free the given downstream qubits.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn free(&mut self, qubits: Vec<QubitRef>) -> Result<()> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("free() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("free() cannot be called while handling a gatestream response")?;
        }
        self.check_qubits_live(qubits.iter())?;

        // Send the free message.
        self.connection
            .send(OutgoingMessage::Downstream(GatestreamDown::Pipelined(
                self.downstream_sequence_tx.get_next(),
                PipelinedGatestreamDown::Free(qubits.clone()),
            )))?;

        // Kill our classical storage for the qubits.
        for qubit in qubits.iter() {
            self.downstream_qubit_data.remove(qubit);
        }

        // Keep the qubit ref generator in sync.
        self.downstream_qubit_ref_generator.free(qubits);

        Ok(())
    }

    /// Tells the downstream plugin to execute a gate.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn gate(&mut self, gate: Gate) -> Result<()> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("gate() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("gate() cannot be called while handling a gatestream response")?;
        }
        self.check_qubits_live(gate.get_targets())?;
        self.check_qubits_live(gate.get_controls())?;
        self.check_qubits_live(gate.get_measures())?;

        // Store which qubits we're expecting to be measured.
        let measures: HashSet<_> = gate.get_measures().iter().cloned().collect();

        // Send the gate message.
        self.connection
            .send(OutgoingMessage::Downstream(GatestreamDown::Pipelined(
                self.downstream_sequence_tx.get_next(),
                PipelinedGatestreamDown::Gate(gate),
            )))?;
        let sequence = self.downstream_sequence_tx.get_previous();

        // Update the last-mutation sequence number for the measured qubits.
        for measure in measures.iter() {
            self.downstream_qubit_data
                .get_mut(measure)
                .unwrap()
                .last_mutation = sequence;
        }

        // Store which measurements we're expecting.
        if !measures.is_empty() {
            self.downstream_expected_measurements
                .push_back((sequence, measures));
        }

        Ok(())
    }

    /// Returns the latest measurement of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_measurement(&mut self, qubit: QubitRef) -> Result<QubitMeasurementResult> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("get_measurement() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op(
                "get_measurement() cannot be called while handling a gatestream response",
            )?;
        }

        // Check that we have data for the qubit, and synchronize up to its
        // last modification.
        if let Some(last_mutation) = self
            .downstream_qubit_data
            .get(&qubit)
            .map(|data| data.last_mutation)
        {
            self.synchronize_downstream_up_to(last_mutation)?;
        } else {
            inv_arg(format!("qubit {} is not allocated", qubit))?;
        }

        // Get the data (possibly updated by `synchronize_downstream_up_to()`
        // and return it.
        let data = &self.downstream_qubit_data[&qubit];
        if let Some(measurement) = &data.measurement {
            Ok(QubitMeasurementResult::new(
                qubit,
                measurement.value,
                measurement.data.clone(),
            ))
        } else {
            inv_arg(format!("qubit {} has not been measured yet", qubit))
        }
    }

    /// Returns the number of downstream cycles since the latest measurement
    /// of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycles_since_measure(&mut self, qubit: QubitRef) -> Result<u64> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("get_cycles_since_measure() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op(
                "get_cycles_since_measure() cannot be called while handling a gatestream response",
            )?;
        }

        // Check that we have data for the qubit, and synchronize up to its
        // last modification.
        if let Some(last_mutation) = self
            .downstream_qubit_data
            .get(&qubit)
            .map(|data| data.last_mutation)
        {
            self.synchronize_downstream_up_to(last_mutation)?;
        } else {
            inv_arg(format!("qubit {} is not allocated", qubit))?;
        }

        // Get the data (possibly updated by `synchronize_downstream_up_to()`
        // and return it.
        let data = &self.downstream_qubit_data[&qubit];
        if let Some(measurement) = &data.measurement {
            let delta = self.downstream_cycle_tx - measurement.timestamp;
            assert!(delta >= 0);
            Ok(delta as u64)
        } else {
            inv_arg(format!("qubit {} has not been measured yet", qubit))
        }
    }

    /// Returns the number of downstream cycles between the last two
    /// measurements of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycles_between_measures(&mut self, qubit: QubitRef) -> Result<u64> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("get_cycles_between_measures() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("get_cycles_between_measures() cannot be called while handling a gatestream response")?;
        }

        // Check that we have data for the qubit, and synchronize up to its
        // last modification.
        if let Some(last_mutation) = self
            .downstream_qubit_data
            .get(&qubit)
            .map(|data| data.last_mutation)
        {
            self.synchronize_downstream_up_to(last_mutation)?;
        } else {
            inv_arg(format!("qubit {} is not allocated", qubit))?;
        }

        // Get the data (possibly updated by `synchronize_downstream_up_to()`
        // and return it.
        let data = &self.downstream_qubit_data[&qubit];
        if let Some(measurement) = &data.measurement {
            if let Some(timer) = measurement.timer {
                Ok(timer)
            } else {
                inv_arg(format!("qubit {} has only been measured once", qubit))
            }
        } else {
            inv_arg(format!("qubit {} has not been measured yet", qubit))
        }
    }

    /// Tells the downstream plugin to run for the specified number of cycles.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn advance(&mut self, cycles: Cycles) -> Result<Cycle> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("advance() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("advance() cannot be called while handling a gatestream response")?;
        }

        // Advance our local counter.
        self.downstream_cycle_tx = self.downstream_cycle_tx.advance(cycles);

        // Send the advance message.
        self.connection
            .send(OutgoingMessage::Downstream(GatestreamDown::Pipelined(
                self.downstream_sequence_tx.get_next(),
                PipelinedGatestreamDown::Advance(cycles),
            )))?;

        // Return the current simulation time.
        Ok(self.downstream_cycle_tx)
    }

    /// Returns the current value of the downstream cycle counter.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycle(&self) -> Result<Cycle> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("get_cycle() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("get_cycle() cannot be called while handling a gatestream response")?;
        }
        Ok(self.downstream_cycle_tx)
    }

    /// Sends an arbitrary command downstream.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn arb(&mut self, cmd: ArbCmd) -> Result<ArbData> {
        if self.definition.get_type() == PluginType::Backend {
            return inv_op("arb() is not available for backends")?;
        } else if !self.synchronized_to_rpcs {
            return inv_op("arb() cannot be called while handling a gatestream response")?;
        }

        // ArbCmds are synchronous in nature, because they return data
        // immediately. Therefore we must first wait for all pipelined
        // requests to complete.
        self.synchronize_downstream()?;

        // Send the command.
        self.connection
            .send(OutgoingMessage::Downstream(GatestreamDown::ArbRequest(cmd)))?;

        // The next downstream response must either be ArbFailure for an error
        // or ArbSuccess for success. Any other message is a protocol error.
        match self.connection.next_downstream_request()? {
            Some(IncomingMessage::Downstream(GatestreamUp::ArbSuccess(x))) => Ok(x),
            Some(IncomingMessage::Downstream(GatestreamUp::ArbFailure(e))) => err(e),
            Some(IncomingMessage::Downstream(_)) => {
                err("Protocol error: unexpected message from downstream")
            }
            Some(_) => panic!("next_downstream_request() returned a non-downstream message"),
            None => err("Simulation aborted"),
        }
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
