use crate::common::{
    error::{inv_arg, Result},
    types::{ArbData, Matrix, QubitRef},
};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, convert::TryInto};

/// Represents a type of quantum or mixed quantum-classical gate.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]
pub enum GateType {
    /// Unitary gates have one or more target qubits, zero or more control
    /// qubits, and a unitary matrix, sized for the number of target qubits.
    ///
    /// The semantics are that the unitary matrix expanded by the number of
    /// control qubits is applied to the qubits.
    ///
    /// The data field may add pragma-like hints to the gate, for instance to
    /// represent the line number in the source file that generated the gate,
    /// error modelling information, and so on. This data may be silently
    /// ignored.
    Unitary,

    /// Measurement gates have one or more measured qubits and a 2x2 unitary
    /// matrix representing the basis.
    ///
    /// The semantics are:
    ///
    ///  - the hermetian of the matrix is applied to each individual qubit;
    ///  - each individual qubit is measured in the Z basis;
    ///  - the matrix is applied to each individual qubit;
    ///  - the results of the measurement are propagated upstream.
    ///
    /// This allows any measurement basis to be used.
    ///
    /// The data field may add pragma-like hints to the gate, for instance to
    /// represent the line number in the source file that generated the gate,
    /// error modelling information, and so on. This data may be silently
    /// ignored.
    Measurement,

    /// Prep gates have one or more target qubits and a 2x2 unitary matrix
    /// representing the basis.
    ///
    /// The semantics are:
    ///
    ///  - each qubit is initialized to |0>;
    ///  - the matrix is applied to each individual qubit.
    ///
    /// This allows any initial state to be used.
    ///
    /// The data field may add pragma-like hints to the gate, for instance to
    /// represent the line number in the source file that generated the gate,
    /// error modelling information, and so on. This data may be silently
    /// ignored.
    Prep,

    /// Custom gates perform a user-defined mixed quantum-classical operation,
    /// identified by a name. They can have zero or more target, control, and
    /// measured qubits, of which only the target and control sets must be
    /// mutually exclusive. They also have an optional matrix of arbitrary
    /// size.
    ///
    /// The semantics are:
    ///
    ///  - if the name is not recognized, an error is reported;
    ///  - a user-defined operation is performed based on the name, qubits,
    ///    matrix, and data arguments;
    ///  - exactly one measurement result is reported upstream for exactly the
    ///    qubits in the measures set.
    Custom(String),
}

/// Represents a type of quantum or mixed quantum-classical gate.
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]
pub struct Gate {
    /// Type of gate. See enum definition. The significance of the targets,
    /// controls, measures, and matrix fields is documented there.
    typ: GateType,

    /// The list of qubits targetted by this gate.
    targets: Vec<QubitRef>,

    /// The set of qubits that control this gate.
    controls: Vec<QubitRef>,

    /// The set of qubits measured by this gate.
    measures: Vec<QubitRef>,

    /// An optional matrix.
    matrix: Option<Matrix>,

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
        let matrix = Matrix::new(matrix)?;

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
        if matrix.num_qubits() != Some(targets.len()) {
            return inv_arg(format!(
                "the matrix is expected to be sized for {} qubits but has dimension {}",
                targets.len(),
                matrix.dimension()
            ));
        }

        // Validate matrix is unitary.
        if !matrix.approx_unitary(1.0e-6) {
            return inv_arg("provided matrix is not unitary within 1e-6 tolerance");
        }

        // Construct the Gate structure.
        Ok(Gate {
            typ: GateType::Unitary,
            targets,
            controls,
            measures: vec![],
            matrix: Some(matrix),
            data: ArbData::default(),
        })
    }

    /// Constructs a new measurement gate.
    pub fn new_measurement(
        qubits: impl IntoIterator<Item = QubitRef>,
        basis: impl IntoIterator<Item = Complex64>,
    ) -> Result<Gate> {
        let measures: Vec<QubitRef> = qubits.into_iter().collect();
        let matrix = Matrix::new(basis)?;

        // Enforce uniqueness of the qubits.
        let mut set = HashSet::new();
        for qubit in measures.iter() {
            if !set.insert(qubit) {
                return inv_arg(format!("qubit {} is measured more than once", qubit));
            }
        }

        // Check the size of the matrix.
        if matrix.dimension() != 2 {
            return inv_arg(format!(
                "the matrix is expected to be 2x2 but has dimension {}",
                matrix.dimension(),
            ));
        }

        // Validate matrix is unitary.
        if !matrix.approx_unitary(1.0e-6) {
            return inv_arg("provided matrix is not unitary within 1e-6 tolerance");
        }

        // Construct the Gate structure.
        Ok(Gate {
            typ: GateType::Measurement,
            targets: vec![],
            controls: vec![],
            measures,
            matrix: Some(matrix),
            data: ArbData::default(),
        })
    }

    /// Constructs a new prep gate.
    pub fn new_prep(
        qubits: impl IntoIterator<Item = QubitRef>,
        matrix: impl IntoIterator<Item = Complex64>,
    ) -> Result<Gate> {
        let targets: Vec<QubitRef> = qubits.into_iter().collect();
        let matrix = Matrix::new(matrix)?;

        // Enforce uniqueness of the qubits.
        let mut set = HashSet::new();
        for qubit in targets.iter() {
            if !set.insert(qubit) {
                return inv_arg(format!("qubit {} is measured more than once", qubit));
            }
        }

        // Check the size of the matrix.
        if matrix.dimension() != 2 {
            return inv_arg(format!(
                "the matrix is expected to be 2x2 but has dimension {}",
                matrix.dimension(),
            ));
        }

        // Validate matrix is unitary.
        if !matrix.approx_unitary(1.0e-6) {
            return inv_arg("provided matrix is not unitary within 1e-6 tolerance");
        }

        // Construct the Gate structure.
        Ok(Gate {
            typ: GateType::Prep,
            targets,
            controls: vec![],
            measures: vec![],
            matrix: Some(matrix),
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
        let matrix: Option<Matrix> = if let Some(matrix) = matrix {
            let matrix: Vec<Complex64> = matrix.into_iter().collect();
            Some(matrix.try_into()?)
        } else {
            None
        };
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

        // Construct the Gate structure.
        Ok(Gate {
            typ: GateType::Custom(name),
            targets,
            controls,
            measures,
            matrix,
            data,
        })
    }

    /// Returns the gate type.
    pub fn get_type(&self) -> &GateType {
        &self.typ
    }

    /// Returns the name of the gate, if any.
    pub fn get_name(&self) -> Option<&str> {
        if let GateType::Custom(name) = &self.typ {
            Some(&name[..])
        } else {
            None
        }
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
    pub fn get_matrix(&self) -> Option<&Matrix> {
        self.matrix.as_ref()
    }

    /// Returns a new Gate with its controls moved to the matrix.
    pub fn with_matrix_controls(&self) -> Self {
        let num_controls = self.controls.len();
        if self.typ == GateType::Unitary && num_controls > 0 {
            let matrix = self.matrix.as_ref().unwrap().add_controls(num_controls);
            let mut targets = self.controls.clone();
            targets.append(&mut self.targets.clone());
            Gate {
                typ: self.typ.clone(),
                targets,
                controls: vec![],
                measures: self.measures.to_vec(),
                matrix: Some(matrix),
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
        if self.typ == GateType::Unitary {
            let matrix = self.matrix.as_ref().unwrap();
            let (control_set, matrix) = matrix.strip_control(epsilon, ignore_global_phase);
            let mut targets = self.get_targets().to_vec();
            let mut controls = vec![];
            for c in control_set {
                controls.push(targets.remove(c));
            }
            Gate {
                typ: self.typ.clone(),
                targets,
                controls,
                measures: self.measures.to_vec(),
                matrix: Some(matrix),
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
            typ: self.typ.clone(),
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
            "Invalid argument: matrix is not square"
        );

        let targets = vec![qref(2), qref(3)];
        let controls = vec![];
        let matrix = vec![Complex64::new(1f64, 1f64)];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be sized for 2 qubits but has dimension 1"
        );

        let targets = vec![qref(1), qref(2), qref(3)];
        let controls = vec![];
        let matrix = vec![Complex64::new(1f64, 1f64)];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be sized for 3 qubits but has dimension 1"
        );

        let targets = vec![qref(1), qref(2), qref(3), qref(4)];
        let controls = vec![];
        let matrix = vec![Complex64::new(1f64, 1f64)];
        let g = Gate::new_unitary(targets, controls, matrix);
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: the matrix is expected to be sized for 4 qubits but has dimension 1"
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
                &vec![
                    Complex64::new(1f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(1f64, 0f64),
                ]
                .try_into()
                .unwrap()
            )
        );
    }

    #[test]
    fn new_measurement_dup_qubit() {
        let g = Gate::new_measurement(vec![qref(1), qref(1)], Matrix::new_identity(2));
        assert!(g.is_err());
        assert_eq!(
            g.unwrap_err().to_string(),
            "Invalid argument: qubit 1 is measured more than once"
        );
    }

    #[test]
    fn new_measurement() {
        let g = Gate::new_measurement(vec![qref(1)], Matrix::new_identity(2));
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.get_name(), None);
        assert_eq!(g.get_targets(), []);
        assert_eq!(g.get_controls(), []);
        assert_eq!(g.get_measures(), [qref(1)]);
        assert_eq!(g.get_matrix(), Some(&Matrix::new_identity(2)));
    }

    #[test]
    fn new_prep() {
        let g = Gate::new_prep(vec![qref(1)], Matrix::new_identity(2));
        assert!(g.is_ok());
        let g = g.unwrap();
        assert_eq!(g.get_name(), None);
        assert_eq!(g.get_targets(), [qref(1)]);
        assert_eq!(g.get_controls(), []);
        assert_eq!(g.get_measures(), []);
        assert_eq!(g.get_matrix(), Some(&Matrix::new_identity(2)));
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
            "Invalid argument: matrix is not square"
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
                &vec![
                    Complex64::new(1f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(0f64, 0f64),
                    Complex64::new(1f64, 0f64),
                ]
                .try_into()
                .unwrap()
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
        assert_eq!(format!("{:?}", g), "Gate { typ: Custom(\"I\"), targets: [QubitRef(1)], controls: [QubitRef(2)], measures: [QubitRef(3)], matrix: Some(Matrix { data: [Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }], dimension: 2 }), data: ArbData { json: Map({}), args: [] } }");
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
        assert_eq!(serde_json::to_string(&g).unwrap(), "{\"typ\":\"Unitary\",\"targets\":[1],\"controls\":[2],\"measures\":[],\"matrix\":{\"data\":[{\"re\":1.0,\"im\":0.0},{\"re\":0.0,\"im\":0.0},{\"re\":0.0,\"im\":0.0},{\"re\":1.0,\"im\":0.0}],\"dimension\":2},\"data\":{\"cbor\":[160],\"args\":[]}}");
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
