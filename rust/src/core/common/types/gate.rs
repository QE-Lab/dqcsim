use crate::common::{
    error::{inv_arg, Result},
    types::{ArbData, Matrix, QubitRef},
};
use float_cmp::approx_eq;
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents a quantum gate.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]
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
    matrix: Matrix,

    /// User-defined classical data to pass along with the gate.
    pub data: ArbData,
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
        let matrix: Vec<Complex64> = matrix.into_iter().collect();

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
        let expected_size = 2usize.pow(2 * targets.len() as u32);
        if matrix.len() != expected_size {
            return inv_arg(format!(
                "the matrix is expected to be of size {} but was {}",
                expected_size,
                matrix.len()
            ));
        }

        // Validate matrix is unitary.
        let dimension = 2usize.pow(targets.len() as u32);

        // Get the complex conjugate transpose.
        let mut conjugate_transpose = vec![Complex64::new(0., 0.); matrix.len()];
        for j in 0..dimension {
            for i in 0..dimension {
                conjugate_transpose[j + i * dimension] = matrix[i + j * dimension].conj();
            }
        }

        // Check if result of matrix multiplication is identity matrix.
        for i in 0..dimension {
            for j in 0..dimension {
                let element = (0..dimension).fold(Complex64::new(0., 0.), |acc, k| {
                    acc + (matrix[i * dimension + k] * conjugate_transpose[k * dimension + j])
                });
                let value = if i == j {
                    Complex64::new(1., 0.)
                } else {
                    Complex64::new(0., 0.)
                };
                if !approx_eq!(f64, element.re, value.re) || !approx_eq!(f64, element.im, value.im)
                {
                    return inv_arg("provided matrix is non-unitary");
                }
            }
        }

        // Construct the Gate structure.
        Ok(Gate {
            name: None,
            targets,
            controls,
            measures: vec![],
            matrix: matrix.into(),
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
            matrix: vec![].into(),
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
        let matrix: Option<Matrix> = matrix.map(|m| m.into_iter().collect());
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
                let expected_size = 2usize.pow(2 * targets.len() as u32);
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
            matrix: matrix.unwrap_or_else(|| vec![].into()),
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
    pub fn get_matrix(&self) -> Option<Matrix> {
        if self.matrix.is_empty() {
            None
        } else {
            Some(self.matrix.clone())
        }
    }

    /// Returns a new Gate with its controls moved to the matrix.
    pub fn with_matrix_controls(&self) -> Self {
        let num_controls = self.controls.len();
        if num_controls > 0 {
            let matrix = self.matrix.add_controls(num_controls);
            let mut targets = self.controls.clone();
            targets.append(&mut self.targets.clone());
            Gate {
                name: self.name.clone(),
                targets,
                controls: vec![],
                measures: self.measures.to_vec(),
                matrix,
                data: self.data.clone(),
            }
        } else {
            self.clone()
        }
    }

    /// Returns a new Gate with controls encoded in the matrix moved to the
    /// Gate controls field. Forwards the epsilon and ignore_global_phase args
    /// to the Matrix::strip_control method.
    pub fn with_gate_controls(&self, epsilon: f64, ignore_global_phase: bool) -> Self {
        if let Some(matrix) = self.get_matrix() {
            let (control_set, matrix) = matrix.strip_control(epsilon, ignore_global_phase);
            let mut targets = self.get_targets().to_vec();
            let mut controls = vec![];
            for c in control_set {
                controls.push(targets.remove(c));
            }
            Gate {
                name: self.name.clone(),
                targets,
                controls,
                measures: self.measures.to_vec(),
                matrix,
                data: self.data.clone(),
            }
        } else {
            self.clone()
        }
    }

    /// Replaces all qubit references in the gate with undefined qubits. This
    /// is used as a gate detector cache preprocessing step when the detector
    /// functions do not depend on which qubits are bound to the gate, only the
    /// amount of each kind,
    pub fn without_qubit_refs(&self) -> Self {
        Gate {
            name: self.name.clone(),
            targets: vec![QubitRef::null(); self.targets.len()],
            controls: vec![QubitRef::null(); self.controls.len()],
            measures: vec![QubitRef::null(); self.measures.len()],
            matrix: self.matrix.clone(),
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qref(q: u64) -> QubitRef {
        QubitRef::from_foreign(q).unwrap()
    }

    #[test]
    fn new_unitary_no_targets() {
        let targets = vec![];
        let controls = vec![];
        let matrix = vec![];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: at least one target qubit is required"
        );
    }

    #[test]
    fn new_unitary_dup_target() {
        let targets = vec![qref(1), qref(1)];
        let controls = vec![];
        let matrix = vec![];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is used more than once"
        );

        let targets = vec![qref(1)];
        let controls = vec![qref(1)];
        let matrix = vec![];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is used more than once"
        );
    }

    #[test]
    fn new_unitary_bad_matrix_size() {
        let targets = vec![qref(2)];
        let controls = vec![qref(1)];
        let matrix = vec![
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
        ];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be of size 4 but was 3"
        );

        let targets = vec![qref(2), qref(3)];
        let controls = vec![];
        let matrix = vec![Complex64::new(1f64, 1f64)];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be of size 16 but was 1"
        );

        let targets = vec![qref(1), qref(2), qref(3)];
        let controls = vec![];
        let matrix = vec![Complex64::new(1f64, 1f64)];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be of size 64 but was 1"
        );

        let targets = vec![qref(1), qref(2), qref(3), qref(4)];
        let controls = vec![];
        let matrix = vec![Complex64::new(1f64, 1f64)];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be of size 256 but was 1"
        );
    }

    #[test]
    fn new_unitary() {
        let targets = vec![qref(1)];
        let controls = vec![qref(2)];
        let matrix = vec![
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
        ];
        let g = Gate::new_unitary(targets.clone(), controls.clone(), matrix);
        assert!(g.is_err());

        let matrix = vec![
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
        ];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.get_name(), None);
        assert_eq!(g.get_targets(), [qref(1)]);
        assert_eq!(g.get_controls(), [qref(2)]);
        assert_eq!(g.get_measures(), []);
        assert_eq!(
            g.get_matrix(),
            Some(
                vec![
                    Complex64::new(1f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(1f64, 0f64),
                ]
                .into()
            )
        );
    }

    #[test]
    fn new_measurement_dup_qubit() {
        let g = Gate::new_measurement(vec![qref(1), qref(1)]);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is measured more than once"
        );
    }

    #[test]
    fn new_measurement() {
        let g = Gate::new_measurement(vec![qref(1)]);
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.get_name(), None);
        assert_eq!(g.get_targets(), []);
        assert_eq!(g.get_controls(), []);
        assert_eq!(g.get_measures(), [qref(1)]);
        assert_eq!(g.get_matrix(), None);
    }

    #[test]
    fn new_custom_dup_qubit() {
        let name = "";
        let targets = vec![qref(1)];
        let controls = vec![qref(1)];
        let measures = vec![];
        let matrix = Some(vec![
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
        ]);
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is used more than once"
        );

        let name = "";
        let targets = vec![qref(1), qref(1)];
        let controls = vec![];
        let measures = vec![];
        let matrix = Some(vec![
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
        ]);
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is used more than once"
        );
    }

    #[test]
    fn new_custom_dup_measure() {
        let name = "";
        let targets = vec![];
        let controls = vec![];
        let measures = vec![qref(1), qref(1)];
        let matrix: Option<Vec<Complex64>> = None;
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is measured more than once"
        );
    }

    #[test]
    fn new_custom_matrix_no_targets() {
        let name = "";
        let targets = vec![];
        let controls = vec![];
        let measures = vec![];
        let matrix = Some(vec![
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
        ]);
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: cannot specify a matrix when there are no target qubits"
        );
    }

    #[test]
    fn new_custom_bad_size() {
        let name = "";
        let targets = vec![qref(1)];
        let controls = vec![];
        let measures = vec![];
        let matrix = Some(vec![
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
            Complex64::new(1f64, 1f64),
        ]);
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be of size 4 but was 3"
        );
    }

    #[test]
    fn new_custom() {
        let name = "I";
        let targets = vec![qref(1)];
        let controls = vec![qref(2)];
        let measures = vec![qref(3)];
        let matrix = Some(vec![
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
        ]);
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.get_name(), Some("I"));
        assert_eq!(g.get_targets(), [qref(1)]);
        assert_eq!(g.get_controls(), [qref(2)]);
        assert_eq!(g.get_measures(), [qref(3)]);
        assert_eq!(
            g.get_matrix(),
            Some(
                vec![
                    Complex64::new(1f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(1f64, 0f64),
                ]
                .into()
            )
        );
    }

    #[test]
    fn debug() {
        let name = "I";
        let targets = vec![qref(1)];
        let controls = vec![qref(2)];
        let measures = vec![qref(3)];
        let matrix = Some(vec![
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
        ]);
        let data = ArbData::default();
        let g = Gate::new_custom(name, targets, controls, measures, matrix, data);
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(format!("{:?}", g), "Gate { name: Some(\"I\"), targets: [QubitRef(1)], controls: [QubitRef(2)], measures: [QubitRef(3)], matrix: Matrix { data: [Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }], dimension: 2 }, data: ArbData { json: Map({}), args: [] } }");
    }

    #[test]
    fn serde() {
        let targets = vec![qref(1)];
        let controls = vec![qref(2)];
        let matrix = vec![
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
        ];
        let g = Gate::new_unitary(targets, controls, matrix).unwrap();
        assert_eq!(serde_json::to_string(&g).unwrap(), "{\"name\":null,\"targets\":[1],\"controls\":[2],\"measures\":[],\"matrix\":[{\"re\":1.0,\"im\":0.0},{\"re\":0.0,\"im\":0.0},{\"re\":0.0,\"im\":0.0},{\"re\":1.0,\"im\":0.0}],\"data\":{\"cbor\":[160],\"args\":[]}}");
    }

    #[test]
    fn with_gate_controls() {
        let targets = vec![qref(1), qref(2)];
        let controls = vec![];
        let matrix = vec![
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            //
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            //
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
            //
            Complex64::new(0f64, 0f64),
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
        ];
        let cnot = Gate::new_unitary(targets, controls, matrix).unwrap();
        assert_eq!(cnot.get_controls(), &[]);
        let x = cnot.with_gate_controls(0.001, false);
        assert_eq!(x.get_controls(), &[qref(1)]);
    }

    #[test]
    fn with_matrix_controls() {
        let targets = vec![qref(1)];
        let controls = vec![qref(2)];
        let matrix = vec![
            Complex64::new(0f64, 0f64),
            Complex64::new(1f64, 0f64),
            Complex64::new(1f64, 0f64),
            Complex64::new(0f64, 0f64),
        ];
        let x = Gate::new_unitary(targets, controls, matrix).unwrap();
        assert_eq!(x.get_controls(), &[qref(2)]);
        let cnot = x.with_matrix_controls();
        assert_eq!(cnot.get_controls(), &[]);
        assert_eq!(cnot.get_targets(), &[qref(2), qref(1)]);
    }
}
