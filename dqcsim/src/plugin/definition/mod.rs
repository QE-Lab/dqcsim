use crate::{
    common::{
        error::{err, Result},
        types::{ArbCmd, ArbData, Gate, PluginMetadata, QubitMeasurementResult, QubitRef},
    },
    host::configuration::PluginType,
    plugin::context::PluginContext,
};
use std::fmt;

/// Defines a plugin.
///
/// This struct is constructed by the user (or the foreign language API). The
/// behavior of the plugin is defined by a number of closures that the user
/// must provide.
#[allow(clippy::type_complexity)]
pub struct PluginDefinition {
    /// Plugin type.
    typ: PluginType,

    /// Name, author, and version of the plugin.
    metadata: PluginMetadata,

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
    ///         measurements.push(QubitMeasurementResult {
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
    pub gate: Box<dyn Fn(&mut PluginContext, Gate) -> Result<Vec<QubitMeasurementResult>>>,

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
    pub modify_measurement: Box<
        dyn Fn(&mut PluginContext, QubitMeasurementResult) -> Result<Vec<QubitMeasurementResult>>,
    >,

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

impl fmt::Debug for PluginDefinition {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PluginDefinition")
            .field("typ", &self.typ)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl PluginDefinition {
    /// Constructs a new plugin definition with default callbacks.
    ///
    /// The callbacks can be overridden by modifying the boxed callback fields
    /// directly.
    pub fn new(typ: PluginType, metadata: impl Into<PluginMetadata>) -> PluginDefinition {
        match typ {
            PluginType::Frontend => PluginDefinition {
                typ: PluginType::Frontend,
                metadata: metadata.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("run() is not implemented")),
                allocate: Box::new(|_, _, _| err("frontend.allocate() called")),
                free: Box::new(|_, _| err("frontend.free() called")),
                gate: Box::new(|_, _| err("frontend.gate() called")),
                modify_measurement: Box::new(|_, _| err("frontend.modify_measurement() called")),
                advance: Box::new(|_, _| err("frontend.advance() called")),
                upstream_arb: Box::new(|_, _| err("frontend.upstream_arb() called")),
                host_arb: Box::new(|_, _| Ok(ArbData::default())),
            },
            PluginType::Operator => PluginDefinition {
                typ: PluginType::Operator,
                metadata: metadata.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("operator.run() called")),
                allocate: Box::new(|ctxt, qubits, cmds| {
                    ctxt.allocate(qubits.len(), cmds).map(|_| ())
                }),
                free: Box::new(|ctxt, qubits| ctxt.free(qubits)),
                gate: Box::new(|ctxt, gate| ctxt.gate(gate).map(|_| vec![])),
                modify_measurement: Box::new(|_, measurement| Ok(vec![measurement])),
                advance: Box::new(|ctxt, cycles| ctxt.advance(cycles).map(|_| ())),
                upstream_arb: Box::new(|_, _| Ok(ArbData::default())),
                host_arb: Box::new(|_, _| Ok(ArbData::default())),
            },
            PluginType::Backend => PluginDefinition {
                typ: PluginType::Backend,
                metadata: metadata.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("backend.run() called")),
                allocate: Box::new(|_, _, _| Ok(())),
                free: Box::new(|_, _| Ok(())),
                gate: Box::new(|_, _| err("gate() is not implemented")),
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

    /// Returns the plugin metadata.
    pub fn get_metadata(&self) -> &PluginMetadata {
        &self.metadata
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
