use crate::common::{
    error::{err, Result},
    log,
    protocol::{ArbCmd, ArbData, Gate, QubitRef},
};

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
    pub fn gate(&mut self, _gate: Gate) -> Result<()> {
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
