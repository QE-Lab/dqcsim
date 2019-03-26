use crate::{
    common::{
        error::{err, Result},
        log,
        protocol::{ArbCmd, ArbData},
    },
    host::configuration::PluginType,
};

/// TODO: move me?
#[repr(transparent)]
pub struct QubitRef(u64);

/// TODO: move me?
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

/// TODO: move me?
pub struct QubitMeasurement {
    pub qubit: QubitRef,
    pub value: bool,
    pub data: ArbData,
}

/// Temporary (?) structure that contains the functions that the user may call
/// from the closures in the plugin definition.
///
/// TODO: move me?
pub struct PluginContext {}

impl PluginContext {
    /// Sends a log message to DQCsim by means of a Record structure.
    pub fn log(&self, _record: log::Record) {}

    /// Sends a message to the host.
    ///
    /// It is only legal to call this function from within the `run()`
    /// callback. Any other source will result in an `Err` return value.
    pub fn send(&mut self, _msg: ArbData) -> Result<()> {
        err("not yet implemented")
    }

    /// Waits for a message from the host.
    ///
    /// It is only legal to call this function from within the `run()`
    /// callback. Any other source will result in an `Err` return value.
    pub fn recv(&mut self) -> Result<ArbData> {
        err("not yet implemented")
    }

    /// Allocate the given number of downstream qubits.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn allocate(&mut self, _num_qubits: usize, _arbs: Vec<ArbCmd>) -> Result<Vec<QubitRef>> {
        err("not yet implemented")
    }

    /// Free the given downstream qubits.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn free(&mut self, _qubits: Vec<QubitRef>) -> Result<()> {
        err("not yet implemented")
    }

    /// Tells the downstream plugin to execute a gate.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn gate(
        &mut self,
        _name: Option<String>,
        _targets: Vec<QubitRef>,
        _controls: Vec<QubitRef>,
        _measures: Vec<QubitRef>,
        _matrix: Vec<Complex>,
        _arb: ArbData,
    ) -> Result<()> {
        err("not yet implemented")
    }

    /// Returns the latest measurement of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_measurement(&self, _qubit: QubitRef) -> Result<bool> {
        err("not yet implemented")
    }

    /// Returns the additional data associated with the latest measurement of
    /// the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_measurement_arb(&self, _qubit: QubitRef) -> Result<ArbData> {
        err("not yet implemented")
    }

    /// Returns the number of downstream cycles since the latest measurement
    /// of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycles_since_measure(&self, _qubit: QubitRef) -> Result<usize> {
        err("not yet implemented")
    }

    /// Returns the number of downstream cycles between the last two
    /// measurements of the given downstream qubit.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycles_between_measures(&self, _qubit: QubitRef) -> Result<usize> {
        err("not yet implemented")
    }

    /// Advances the downstream cycle counter.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn advance(&mut self, _cycles: usize) -> Result<usize> {
        err("not yet implemented")
    }

    /// Returns the current value of the downstream cycle counter.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn get_cycle(&self) -> Result<usize> {
        err("not yet implemented")
    }

    /// Sends an arbitrary command downstream.
    ///
    /// Backend plugins are not allowed to call this. Doing so will result in
    /// an `Err` return value.
    pub fn arb(&mut self, _cmd: ArbCmd) -> Result<ArbData> {
        err("not yet implemented")
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
        0
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
        0.0
    }
}

/// Defines a plugin.
///
/// This struct is constructed by the user (or the foreign language API). The
/// behavior of the plugin is defined by a number of closures that the user
/// must provide.
#[allow(clippy::type_complexity)]
pub struct PluginDefinition {
    /// Plugin type.
    typ: PluginType,

    /// Name of the plugin.
    name: String,

    /// Author of the plugin.
    author: String,

    /// Version of the plugin.
    version: String,

    /// Initialization callback.
    ///
    /// This is always called before any of the other callbacks are run. The
    /// downstream plugin has already been initialized at this stage, so it is
    /// legal to send it commands.
    ///
    /// The default behavior is no-op.
    pub initialize: Box<dyn Fn(&mut PluginContext, Vec<ArbCmd>) -> Result<()>>,

    /// Finalization callback.
    ///
    /// This is called when a plugin is gracefully terminated.
    ///
    /// The default behavior is no-op.
    pub drop: Box<dyn Fn(&mut PluginContext) -> Result<()>>,

    /// Run callback for frontends.
    ///
    /// This is called in response to a `start()` host API call. The return
    /// value is returned through the `wait()` host API call.
    ///
    /// The default behavior is to fail with a "not implemented" error;
    /// frontends backends should always override this. This callback is never
    /// called for operator or backend plugins.
    pub run: Box<dyn Fn(&mut PluginContext, ArbData) -> Result<ArbData>>,

    /// Qubit allocation callback for operators and backends.
    ///
    /// The default for operators is to pass through to `ctxt.allocate()`. The
    /// default for backends is no-op. This callback is never called for
    /// frontend plugins.
    pub allocate: Box<dyn Fn(&mut PluginContext, Vec<QubitRef>, Vec<ArbCmd>) -> Result<()>>,

    /// Qubit deallocation callback for operators and backends.
    ///
    /// The default for operators is to pass through to `ctxt.free()`. The
    /// default for backends is no-op. This callback is never called for
    /// frontend plugins.
    pub free: Box<dyn Fn(&mut PluginContext, Vec<QubitRef>) -> Result<()>>,

    /// Gate execution callback for operators and backends.
    ///
    /// Backends must return any measured qubits through the return value. Only
    /// qubits that are part of the `measures` vector may be returned in this
    /// vector.
    ///
    /// The story is more complicated for operators. Let's say we want to make
    /// a silly operator that inverts all measurements. While the trivial way
    /// to do this would be something like:
    ///
    /// ```ignore
    /// fn gate(...) -> ... {
    ///     ctxt.gate(...)?;
    ///     let mut measurements;
    ///     for qubit in measures {
    ///         measurements.push(QubitMeasurement {
    ///             qubit,
    ///             value: !ctxt.get_measurement(qubit)?, // note the !
    ///             data: ctxt.get_measurement_arb(qubit)?,
    ///         });
    ///     }
    ///     Ok(measurements)
    /// }
    /// ```
    ///
    /// this pattern should be avoided, because `get_measurement()` prevents
    /// the operator thread from continuing until the downstream plugin has
    /// returned the measurement, which may slow down the simulation. Instead,
    /// you can return an empty vector (or leave the function unimplemented as
    /// this is its default behavior) and use `modify_measurements()` for this:
    ///
    /// ```ignore
    /// fn gate(...) -> ... {
    ///     ctxt.gate(...)?;
    /// }
    ///
    /// fn modify_measurement(...) -> ... {
    ///     measurement.value = !measurement.value;
    ///     Ok(vec![measurement])
    /// }
    /// ```
    ///
    /// `gate()` is called when we get the request to execute a gate from
    /// upstream, while `modify_measurement()` is called when we receive the
    /// result from downstream.
    ///
    /// For more complex operators this method may become unwieldly. It's still
    /// fine to implement everything in `gate()` in this case. However, if you
    /// do this, override `modify_measurement()` to always return an empty
    /// vector, otherwise the two methods may interfere with each other!
    ///
    /// The default behavior for operators is to pass through to `ctxt.gate()`.
    /// An empty measurements vector.is returned in this case; the measurements
    /// are instead propagated through the `modify_measurements()` callback.
    /// The default for backends is to fail with a "not implemented" error;
    /// backends should always override this. This callback is never called for
    /// frontend plugins.
    pub gate: Box<
        dyn Fn(
            &mut PluginContext,
            Option<String>,
            Vec<QubitRef>,
            Vec<QubitRef>,
            Vec<QubitRef>,
            Vec<Complex>,
            ArbData,
        ) -> Result<Vec<QubitMeasurement>>,
    >,
    /// Measurement modification callback for operators.
    ///
    /// This callback is somewhat special, in that it is not allowed to call
    /// any plugin command other than logging and the pseudorandom number
    /// generator functions. This is because this function is called
    /// asynchronously with respect to the downstream functions, making the
    /// timing of these calls non-deterministic based on operating system
    /// scheduling.
    ///
    /// Note that while this function is called for only a single measurement
    /// at a time, it is allowed to produce a vector of measurements. This
    /// allows you to cancel propagation of the measurement by returning an
    /// empty vector, to just modify the measurement data itself, or to
    /// generate additional measurements from a single measurement. However,
    /// if you need to modify the qubit references for operators that remap
    /// qubits, take care to only send measurement data upstream when this is
    /// explicitly requested through a gate function's `measured` list.
    ///
    /// The default behavior for this callback is to return the measurement
    /// without modification.
    pub modify_measurement:
        Box<dyn Fn(&mut PluginContext, QubitMeasurement) -> Result<Vec<QubitMeasurement>>>,

    /// Callback for advancing time for operators and backends.
    ///
    /// The default behavior for operators is to pass through to
    /// `ctxt.advance()`. The default for backends is no-op. This callback is
    /// never called for frontend plugins.
    pub advance: Box<dyn Fn(&mut PluginContext, usize) -> Result<()>>,

    /// Callback function for handling an arb from upstream for operators and
    /// backends.
    ///
    /// The default behavior for operators is to pass through to
    /// `ctxt.arb()`; operators that do not support the requested interface
    /// should always do this. The default for backends is no-op. This callback
    /// is never called for frontend plugins.
    pub upstream_arb: Box<dyn Fn(&mut PluginContext, ArbCmd) -> Result<ArbData>>,

    /// Callback function for handling an arb from the host.
    ///
    /// The default behavior for this is no-op.
    pub host_arb: Box<dyn Fn(&mut PluginContext, ArbCmd) -> Result<ArbData>>,
}

impl PluginDefinition {
    /// Constructs a new plugin definition with default callbacks.
    ///
    /// The callbacks can be overridden by modifying the boxed callback fields
    /// directly.
    pub fn new(
        typ: PluginType,
        name: impl Into<String>,
        author: impl Into<String>,
        version: impl Into<String>,
    ) -> PluginDefinition {
        match typ {
            PluginType::Frontend => PluginDefinition {
                typ: PluginType::Frontend,
                name: name.into(),
                author: author.into(),
                version: version.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("run() is not implemented")),
                allocate: Box::new(|_, _, _| err("frontend.allocate() called")),
                free: Box::new(|_, _| err("frontend.free() called")),
                gate: Box::new(|_, _, _, _, _, _, _| err("frontend.gate() called")),
                modify_measurement: Box::new(|_, _| err("frontend.modify_measurement() called")),
                advance: Box::new(|_, _| err("frontend.advance() called")),
                upstream_arb: Box::new(|_, _| err("frontend.upstream_arb() called")),
                host_arb: Box::new(|_, _| Ok(ArbData::default())),
            },
            PluginType::Operator => PluginDefinition {
                typ: PluginType::Operator,
                name: name.into(),
                author: author.into(),
                version: version.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("operator.run() called")),
                allocate: Box::new(|ctxt, qubits, cmds| {
                    ctxt.allocate(qubits.len(), cmds).map(|_| ())
                }),
                free: Box::new(|ctxt, qubits| ctxt.free(qubits)),
                gate: Box::new(|ctxt, name, targets, controls, measures, matrix, data| {
                    ctxt.gate(name, targets, controls, measures, matrix, data)
                        .map(|_| vec![])
                }),
                modify_measurement: Box::new(|_, measurement| Ok(vec![measurement])),
                advance: Box::new(|ctxt, cycles| ctxt.advance(cycles).map(|_| ())),
                upstream_arb: Box::new(|_, _| Ok(ArbData::default())),
                host_arb: Box::new(|_, _| Ok(ArbData::default())),
            },
            PluginType::Backend => PluginDefinition {
                typ: PluginType::Backend,
                name: name.into(),
                author: author.into(),
                version: version.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("backend.run() called")),
                allocate: Box::new(|_, _, _| Ok(())),
                free: Box::new(|_, _| Ok(())),
                gate: Box::new(|_, _, _, _, _, _, _| err("gate() is not implemented")),
                modify_measurement: Box::new(|_, _| err("backend.modify_measurement() called")),
                advance: Box::new(|_, _| Ok(())),
                upstream_arb: Box::new(|_, _| Ok(ArbData::default())),
                host_arb: Box::new(|_, _| Ok(ArbData::default())),
            },
        }
    }

    /// Returns the plugin type.
    pub fn get_type(&self) -> PluginType {
        self.typ
    }

    /// Returns the plugin name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the plugin author.
    pub fn get_author(&self) -> &str {
        &self.author
    }

    /// Returns the plugin version.
    pub fn get_version(&self) -> &str {
        &self.version
    }

    /// Executes the plugin using the previously specified callback functions.
    ///
    /// `argv` should be set to the command line argument vector, including the
    /// program name/argv[0]. If no program name is available, just make
    /// something up to put there. If the return value is an `Err`, its message
    /// should be printed to `stderr` and exit code 1 should be used.
    /// If `Ok`, nothing should be printed, and the contained value specifies
    /// the exit code.
    pub fn execute(self, _argv: Vec<std::ffi::OsString>) -> Result<i32> {
        err("not yet implemented")
    }
}
