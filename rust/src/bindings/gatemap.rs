use super::*;
use crate::common::{
    detector::{Detector, DetectorMap},
    gates::GateType,
};

/// Rust representation of the user-defined parameters needed to construct a
/// gate.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BoundUserGate {
    /// The constructor function key, i.e. the type of gate.
    key: UserKey,
    /// The qubits that the gate operates on.
    qubits: Vec<QubitRef>,
    /// Classical parameterization data for the gate, such as rotation angles
    /// or error parameters.
    data: ArbData,
}

/// A GateMap to detect and construct Gates.
#[derive(Debug)]
pub struct GateMap<'detectors> {
    ignore_qubit_refs: bool,
    ignore_data: bool,
    key_cmp: Option<extern "C" fn(*const c_void, *const c_void) -> bool>,
    key_hash: Option<extern "C" fn(*const c_void) -> u64>,
    detector: DetectorMap<'detectors, UserKey, Gate, ArbData>,
    constructor: DetectorMap<'detectors, (), BoundUserGate, Gate>,
}

impl<'gm> GateMap<'gm> {
    /// Constructs a new empty GateMap.
    pub fn new(
        ignore_qubit_refs: bool,
        ignore_data: bool,
        key_cmp: Option<extern "C" fn(*const c_void, *const c_void) -> bool>,
        key_hash: Option<extern "C" fn(*const c_void) -> u64>,
    ) -> Self {
        GateMap {
            ignore_qubit_refs,
            ignore_data,
            key_cmp,
            key_hash,
            detector: DetectorMap::new(),
            constructor: DetectorMap::new(),
        }
    }

    fn make_user_key(&self, data: UserKeyData) -> UserKey {
        UserKey::new(data, self.key_cmp, self.key_hash)
    }

    /// Inserts a unitary gate mapping using DQCsim gate types.
    pub fn add_predefined_unitary(
        &mut self,
        key: UserKeyData,
        gate_type: GateType,
        num_controls: Option<usize>,
        epsilon: f64,
        ignore_global_phase: bool,
    ) {
        let gate_detector =
            GateTypeDetector::new(gate_type, num_controls, ignore_global_phase, epsilon);
        self.detector.push(self.make_user_key(key), gate_detector);
        // self.constructor.push();
    }

    /// Inserts a unitary gate mapping using a fixed matrix.
    pub fn add_fixed_unitary(
        &mut self,
        _key: UserKeyData,
        _matrix: Matrix,
        _num_controls: Option<usize>,
        _epsilon: f64,
        _ignore_global_phase: bool,
    ) {
        todo!();
    }

    /// Inserts a unitary gate mapping using a fixed matrix.
    pub fn add_measure(&mut self, _key: UserKeyData, _num_measures: Option<usize>) {
        todo!();
    }

    pub fn detect(&self, _gate: &Gate) -> Result<Option<(UserKey, ArbData)>> {
        todo!();
        //     if let Some(matrix) = gate.get_matrix() {
        //         if self.ignore_data && self.ignore_qubit_refs {
        //             let gate = Gate::new_unitary(vec![], vec![], matrix.into_iter())?;
        //             self.detector.detect(&gate)
        //         } else if self.ignore_qubit_refs {
        //             let mut g = Gate::new_unitary(vec![], vec![], matrix.into_iter())?;
        //             g.data = gate.data.clone();
        //             self.detector.detect(&g)
        //         } else if self.ignore_data {
        //             let mut g = gate.clone();
        //             g.data = ArbData::default();
        //             self.detector.detect(&g)
        //         } else {
        //             self.detector.detect(gate)
        //         }
        //     } else {
        //         Ok(None)
        //     }
    }

    pub fn construct(
        &self,
        _key: UserKeyData,
        _qubits: Vec<QubitRef>,
        _param_data: ArbData,
    ) -> Result<Option<Gate>> {
        todo!();
    }
}

// NOTE(jvs): instead of below, it probably makes a lot more sense to implement
// a MatrixConverter (impl Detector and Constructor) for gates with fixed
// matrices, specific converters for the others (RxConverter, RyConverter,
// RzConverter, RzkConverter, RConverter), a MeasureConverter for
// measurements, and a CustomConverter taking lambdas for constructing the
// converters for the two customizable functions. Or, the way I would
// personally do it, only make the latter and solve it all using lambdas
// in `gm.rs`.

#[derive(Debug)]
struct GateTypeDetector {
    gate_type: GateType,
    num_controls: Option<usize>,
    ignore_global_phase: bool,
    epsilon: f64,
}

impl GateTypeDetector {
    fn new(
        gate_type: GateType,
        num_controls: Option<usize>,
        ignore_global_phase: bool,
        epsilon: f64,
    ) -> Self {
        GateTypeDetector {
            gate_type,
            num_controls,
            ignore_global_phase,
            epsilon,
        }
    }
}

impl Detector for GateTypeDetector {
    type Input = Gate;
    type Output = ArbData;

    fn detect(&self, input: &Self::Input) -> Result<Option<Self::Output>> {
        if let Some(_matrix) = input.get_matrix() {
            todo!()
        } else {
            Ok(None)
        }
    }
}
