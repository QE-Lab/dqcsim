use crate::common::protocol::{ArbData, QubitRef};
//use num_complex::Complex64;
use serde::{Deserialize, Serialize};

/// Represents a complex number internally.
///
/// Unfortunately we can't use Complex64 because it is not (de)serializable.
#[derive(Debug, Serialize, Deserialize)]
struct InternalComplex64 {
    re: f64,
    im: f64,
}

/// Represents a quantum gate.
#[derive(Debug, Serialize, Deserialize)]
pub struct Gate {
    /// Optional name for this gate.
    ///
    /// If this is specified, the behavior of the gate is dependent on the
    /// downstream plugin implementation. This is by design - it allows
    /// users of DQCsim to describe more complex gates than the relatively
    /// limited set specified by DQCsim itself.
    ///
    /// If the name is NOT specified, the gate MUST behave as follows:
    ///
    ///  - if a unitary matrix is supplied:
    ///     - extend the matrix by the amount of control qubits specified;
    ///     - apply the matrix to the concatenation of the control and
    ///       target qubit lists;
    ///  - if target and/or control qubits were specified but no matrix was
    ///    specified, return an error;
    ///  - if the measured qubit list is non-empty, measure the specified
    ///    qubits in the Z basis (i.e., after application of the matrix,
    ///    if any).
    name: Option<String>,

    /// The list of qubits targetted by this gate.
    ///
    /// If a matrix is specified, it must be appropriately sized for the
    /// number of qubits in this vector.
    targets: Vec<QubitRef>,

    /// The set of qubits that control this gate.
    ///
    /// If a matrix is specified, its size is NOT affected by the size of
    /// this set, i.e. the control qubits are implied. For instance, a gate
    /// with the following parameters:
    ///
    ///  - targets: [target qubit]
    ///  - controls: [control qubit]
    ///  - matrix: [0, 1; 1, 0]
    ///
    /// describes a controlled X (a.k.a. CNOT) gate. Plugins are free to
    /// define a CNOT without using the controls set as well, i.e.
    ///
    ///  - targets: [control qubit, target qubit]
    ///  - controls: []
    ///  - matrix: [1, 0, 0, 0; 0, 1, 0, 0; 0, 0, 0, 1; 0, 0, 1, 0]
    ///
    /// is normally equivalent. However, the latter takes a bit more
    /// bandwidth in the communication channel and does not clarify intent
    /// as well as the former does.
    ///
    /// Note that the qubits listed in this set are mutually exclusive with
    /// the target qubits.
    controls: Vec<QubitRef>,

    /// The set of qubits measured by this gate.
    ///
    /// There should be exactly one `GatestreamUp::Measured` message sent
    /// in response for each qubit listed in this set. Failure to do this
    /// results in a warning message being logged and the measurement value
    /// being set to undefined. The reason for this requirement, and the
    /// measured qubits needing to be explicitly specified at all, has to
    /// do with waiting for the downstream plugins to catch up with the
    /// pipelined requests when a measurement result is requested upstream.
    ///
    /// Note that there are no mutual exclusivity constraints between this
    /// set and the targets/controls set. If a qubit is both acted upon and
    /// measured, the measurement is executed after the gate.
    ///
    /// The measurement method (basis, parity, etc.) is not explicitly
    /// specified. It is to be determined based upon the name of the gate
    /// and/or the data object. If no gate name is specified, the Z basis
    /// is implied.
    measures: Vec<QubitRef>,

    /// An optional unitary matrix sized appropriately for the qubits in
    /// `targets`.
    ///
    /// If no gate name is specified, this matrix is applied to the target
    /// qubits (or, if control qubits are specified in addition, the matrix
    /// is first extended to a controlled gate and applied to both the
    /// target and control qubits). However, if a gate name is specified,
    /// it is ultimately up to the downstream plugin how the matrix is
    /// interpreted. For instance, the matrix may be used to specify only a
    /// rotation axis, with the actual rotation amount specified by the
    /// data object. It is up to the user to ensure that the plugins used
    /// within a simulation agree upon the representation used. However,
    /// the size of the matrix is fixed based on the number of target
    /// qubits. If a differently-sized matrix must be communicated, leave
    /// the matrix field unspecified and use the data object instead.
    matrix: Vec<InternalComplex64>,

    /// User-defined classical data to pass along with the gate.
    data: ArbData,
}
