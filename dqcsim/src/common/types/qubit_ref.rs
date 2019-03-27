use libc::size_t;
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

impl QubitRef {
    /// Converts the foreign representation of a qubit reference to the
    /// type-safe Rust representation.
    pub fn from_foreign(qubit: size_t) -> Option<QubitRef> {
        if qubit == 0 {
            None
        } else {
            Some(QubitRef(qubit as usize))
        }
    }

    /// Converts the type-safe Rust representation of a qubit reference to the
    /// foreign representation.
    pub fn to_foreign(self) -> size_t {
        assert_ne!(self.0, 0);
        self.0 as size_t
    }

    /// Converts the type-safe Rust representation of a qubit reference to the
    /// foreign representation.
    pub fn option_to_foreign(qubit: Option<QubitRef>) -> size_t {
        if let Some(x) = qubit {
            x.0 as size_t
        } else {
            0
        }
    }
}

/// Struct used to generate new qubit references.
///
/// Qubit references start at 1; 0 is reserved for representing errors/invalid
/// handles on the foreign language interface. The current implementation just
/// counts references up from 1 when a qubit is allocated, i.e. it does not
/// reuse references.
pub struct QubitRefGenerator {
    counter: std::ops::RangeFrom<usize>,
}

impl Default for QubitRefGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl QubitRefGenerator {
    /// Constructs a new reference generator.
    fn new() -> QubitRefGenerator {
        QubitRefGenerator { counter: (1..) }
    }

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
