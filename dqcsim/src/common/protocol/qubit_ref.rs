use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a reference to a qubit.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QubitRef(usize);

impl fmt::Display for QubitRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Struct used to generate new qubit references.
pub struct QubitRefGenerator {
    counter: std::ops::RangeFrom<usize>,
}

impl Default for QubitRefGenerator {
    fn default() -> Self {
        QubitRefGenerator { counter: (1..) }
    }
}

impl QubitRefGenerator {
    /// "Allocates" a number of qubit references.
    pub fn allocate(&mut self, num_qubits: usize) -> Vec<QubitRef> {
        (&mut self.counter).take(num_qubits).map(QubitRef).collect()
    }

    /// "Frees" a number of qubit references.
    ///
    /// Note that this is no-op in the current implementation; freed qubits are
    /// never reused. This function is defined only in case we want to change
    /// that for some reason.
    pub fn free(&mut self, _qubits: impl IntoIterator<Item = QubitRef>) {
        // Intentionally no-op
    }
}
