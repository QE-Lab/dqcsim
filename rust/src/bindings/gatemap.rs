use super::*;
use crate::common::gates::GateType;

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

#[allow(dead_code)] // TODO
impl BoundUserGate {
    pub fn new(key: UserKey, qubits: Vec<QubitRef>, data: ArbData) -> BoundUserGate {
        BoundUserGate { key, qubits, data }
    }

    pub fn get_key(&self) -> &UserKey {
        &self.key
    }

    pub fn get_qubits(&self) -> &Vec<QubitRef> {
        &self.qubits
    }

    pub fn get_data(&self) -> &ArbData {
        &self.data
    }
}

/// A GateMap to detect and construct Gates.
#[derive(Debug)]
pub struct GateMap<'detectors> {
    ignore_qubit_refs: bool,
    ignore_data: bool,
    key_cmp: Option<extern "C" fn(*const c_void, *const c_void) -> bool>,
    key_hash: Option<extern "C" fn(*const c_void) -> u64>,
    x: std::marker::PhantomData<&'detectors ()>,
    // detector: DetectorMap<'detectors, UserKey, Gate, ArbData>,
    // constructor: DetectorMap<'detectors, (), BoundUserGate, Gate>,
}

impl<'gm> GateMap<'gm> {
    /// Constructs a new empty GateMap.
    #[allow(dead_code)]
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
            x: std::marker::PhantomData,
            // detector: DetectorMap::new(),
            // constructor: DetectorMap::new(),
        }
    }

    #[allow(dead_code)]
    fn make_user_key(&self, data: UserKeyData) -> UserKey {
        UserKey::new(data, self.key_cmp, self.key_hash)
    }

    /// Inserts a unitary gate mapping using DQCsim gate types.
    pub fn add_predefined_unitary(
        &mut self,
        _key: UserKeyData,
        _gate_type: GateType,
        _num_controls: Option<usize>,
        _epsilon: f64,
        _ignore_global_phase: bool,
    ) {
        todo!()
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
