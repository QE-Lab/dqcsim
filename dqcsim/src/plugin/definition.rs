use crate::{
    common::{
        error::{err, Result},
        types::{
            ArbCmd, ArbData, Gate, PluginMetadata, PluginType, QubitMeasurementResult, QubitRef,
        },
    },
    plugin::state::PluginState,
};
use std::fmt;

/// Defines a plugin.
///
/// This struct is constructed by the user (or the foreign language API). The
/// behavior of the plugin is defined by a number of closures that the user
/// must provide. For more information about the callback functions, refer to
/// the documentation of the foreign language API setter functions.
#[allow(clippy::type_complexity)]
pub struct PluginDefinition {
    /// Plugin type.
    typ: PluginType,

    /// Name, author, and version of the plugin.
    metadata: PluginMetadata,

    /// Initialization callback.
    pub initialize: Box<dyn Fn(&mut PluginState, Vec<ArbCmd>) -> Result<()> + Send + 'static>,

    /// Finalization callback.
    pub drop: Box<dyn Fn(&mut PluginState) -> Result<()> + Send + 'static>,

    /// Run callback for frontends.
    pub run: Box<dyn Fn(&mut PluginState, ArbData) -> Result<ArbData> + Send + 'static>,

    /// Qubit allocation callback for operators and backends.
    pub allocate:
        Box<dyn Fn(&mut PluginState, Vec<QubitRef>, Vec<ArbCmd>) -> Result<()> + Send + 'static>,

    /// Qubit deallocation callback for operators and backends.
    pub free: Box<dyn Fn(&mut PluginState, Vec<QubitRef>) -> Result<()> + Send + 'static>,

    /// Gate execution callback for operators and backends.
    pub gate:
        Box<dyn Fn(&mut PluginState, Gate) -> Result<Vec<QubitMeasurementResult>> + Send + 'static>,

    /// Measurement modification callback for operators.
    pub modify_measurement: Box<
        dyn Fn(&mut PluginState, QubitMeasurementResult) -> Result<Vec<QubitMeasurementResult>>
            + Send
            + 'static,
    >,

    /// Callback for advancing time for operators and backends.
    pub advance: Box<dyn Fn(&mut PluginState, u64) -> Result<()> + Send + 'static>,

    /// Callback function for handling an arb from upstream for operators and
    /// backends.
    pub upstream_arb: Box<dyn Fn(&mut PluginState, ArbCmd) -> Result<ArbData> + Send + 'static>,

    /// Callback function for handling an arb from the host.
    pub host_arb: Box<dyn Fn(&mut PluginState, ArbCmd) -> Result<ArbData> + Send + 'static>,
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
                typ,
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
                typ,
                metadata: metadata.into(),
                initialize: Box::new(|_, _| Ok(())),
                drop: Box::new(|_| Ok(())),
                run: Box::new(|_, _| err("operator.run() called")),
                allocate: Box::new(|state, qubits, cmds| {
                    state.allocate(qubits.len(), cmds).map(|_| ())
                }),
                free: Box::new(|state, qubits| state.free(qubits)),
                gate: Box::new(|state, gate| state.gate(gate).map(|_| vec![])),
                modify_measurement: Box::new(|_, measurement| Ok(vec![measurement])),
                advance: Box::new(|state, cycles| state.advance(cycles).map(|_| ())),
                upstream_arb: Box::new(|_, _| Ok(ArbData::default())),
                host_arb: Box::new(|_, _| Ok(ArbData::default())),
            },
            PluginType::Backend => PluginDefinition {
                typ,
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

    /// Sets the type of the plugin.
    pub fn set_type(&mut self, typ: PluginType) {
        self.typ = typ;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let mut def = PluginDefinition::new(
            PluginType::Operator,
            PluginMetadata::new("name", "author", "0.1.0"),
        );
        assert_eq!(
            format!(
                "{:?}",
                def
            ),
            "PluginDefinition { typ: Operator, metadata: PluginMetadata { name: \"name\", author: \"author\", version: \"0.1.0\" } }"
        );
        def.set_type(PluginType::Backend);
        assert_eq!(
            format!(
                "{:?}",
                def
            ),
            "PluginDefinition { typ: Backend, metadata: PluginMetadata { name: \"name\", author: \"author\", version: \"0.1.0\" } }"
        );
    }
}
