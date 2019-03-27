use crate::common::{
    error::{inv_arg, Result},
    types::{ArbData, QubitRef},
};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents a complex number internally.
///
/// Unfortunately we can't use Complex64 because it is not (de)serializable.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct InternalComplex64 {
    re: f64,
    im: f64,
}

/// Represents a quantum gate.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

impl Gate {
    /// Constructs a new unitary gate.
    pub fn new_unitary(
        targets: impl IntoIterator<Item = QubitRef>,
        controls: impl IntoIterator<Item = QubitRef>,
        matrix: impl IntoIterator<Item = Complex64>,
    ) -> Result<Gate> {
        let targets: Vec<QubitRef> = targets.into_iter().collect();
        let controls: Vec<QubitRef> = controls.into_iter().collect();
        let matrix: Vec<InternalComplex64> = matrix
            .into_iter()
            .map(|x| InternalComplex64 { re: x.re, im: x.im })
            .collect();

        // We need at least one target.
        if targets.is_empty() {
            return inv_arg("at least one target qubit is required");
        }

        // Enforce uniqueness of the qubits.
        let mut set = HashSet::new();
        for qubit in targets.iter().chain(controls.iter()) {
            if !set.insert(qubit) {
                return inv_arg(format!("qubit {} is used more than once", qubit));
            }
        }

        // Check the size of the matrix.
        let expected_size = 4 << (targets.len() - 1);
        if matrix.len() != expected_size {
            return inv_arg(format!(
                "the matrix is expected to be of size {} but was {}",
                expected_size,
                matrix.len()
            ));
        }

        // Construct the Gate structure.
        Ok(Gate {
            name: None,
            targets,
            controls,
            measures: vec![],
            matrix,
            data: ArbData::default(),
        })
    }

    /// Constructs a new measurement gate.
    ///
    /// The qubits are measured in the Z basis.
    pub fn new_measurement(qubits: impl IntoIterator<Item = QubitRef>) -> Result<Gate> {
        let measures: Vec<QubitRef> = qubits.into_iter().collect();

        // Enforce uniqueness of the qubits.
        let mut set = HashSet::new();
        for qubit in measures.iter() {
            if !set.insert(qubit) {
                return inv_arg(format!("qubit {} is measured more than once", qubit));
            }
        }

        // Construct the Gate structure.
        Ok(Gate {
            name: None,
            targets: vec![],
            controls: vec![],
            measures,
            matrix: vec![],
            data: ArbData::default(),
        })
    }

    /// Constructs a new implementation-defined gate.
    pub fn new_custom(
        name: impl Into<String>,
        targets: impl IntoIterator<Item = QubitRef>,
        controls: impl IntoIterator<Item = QubitRef>,
        measures: impl IntoIterator<Item = QubitRef>,
        matrix: Option<impl IntoIterator<Item = Complex64>>,
        data: impl Into<ArbData>,
    ) -> Result<Gate> {
        let name: String = name.into();
        let targets: Vec<QubitRef> = targets.into_iter().collect();
        let controls: Vec<QubitRef> = controls.into_iter().collect();
        let measures: Vec<QubitRef> = measures.into_iter().collect();
        let matrix: Option<Vec<InternalComplex64>> = matrix.map(|x| {
            x.into_iter()
                .map(|x| InternalComplex64 { re: x.re, im: x.im })
                .collect()
        });
        let data: ArbData = data.into();

        // Enforce uniqueness of the target/control qubits.
        let mut set = HashSet::new();
        for qubit in targets.iter().chain(controls.iter()) {
            if !set.insert(qubit) {
                return inv_arg(format!("qubit {} is used more than once", qubit));
            }
        }

        // Enforce uniqueness of the measured qubits.
        let mut set = HashSet::new();
        for qubit in measures.iter() {
            if !set.insert(qubit) {
                return inv_arg(format!("qubit {} is measured more than once", qubit));
            }
        }

        // Check the size of the matrix.
        if let Some(ref m) = matrix {
            if targets.is_empty() {
                return inv_arg("cannot specify a matrix when there are no target qubits");
            } else {
                let expected_size = 4 << (targets.len() - 1);
                if m.len() != expected_size {
                    return inv_arg(format!(
                        "the matrix is expected to be of size {} but was {}",
                        expected_size,
                        m.len()
                    ));
                }
            }
        }

        // Construct the Gate structure.
        Ok(Gate {
            name: Some(name),
            targets,
            controls,
            measures,
            matrix: matrix.unwrap_or_else(|| vec![]),
            data,
        })
    }

    /// Returns the name of the gate, if any.
    ///
    /// No name implies DQCsim-defined gate behavior, named gates imply
    /// plugin-defined behavior.
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &x[..])
    }

    /// Returns the list of target qubits.
    pub fn get_targets(&self) -> &[QubitRef] {
        &self.targets
    }

    /// Returns the list of control qubits.
    pub fn get_controls(&self) -> &[QubitRef] {
        &self.controls
    }

    /// Returns the list of measured qubits.
    pub fn get_measures(&self) -> &[QubitRef] {
        &self.measures
    }

    /// Returns the gate matrix.
    pub fn get_matrix(&self) -> Option<Vec<Complex64>> {
        if self.matrix.is_empty() {
            None
        } else {
            Some(
                self.matrix
                    .iter()
                    .map(|x| Complex64 { re: x.re, im: x.im })
                    .collect(),
            )
        }
    }

    /// Returns the user data associated with this gate.
    pub fn get_data(&self) -> &ArbData {
        &self.data
    }
}
