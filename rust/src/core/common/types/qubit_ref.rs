use crate::common::error::{inv_arg, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a reference to a qubit.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QubitRef(u64);

impl fmt::Display for QubitRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for QubitRef {
    fn into(self) -> String {
        format!("{}", self.0)
    }
}

impl QubitRef {
    /// Returns a null qubit reference.
    pub fn null() -> QubitRef {
        QubitRef(0)
    }

    /// Returns whether this reference is null.
    pub fn is_null(self) -> bool {
        self.0 == 0
    }

    /// Converts the foreign representation of a qubit reference to the
    /// type-safe Rust representation.
    pub fn from_foreign(qubit: u64) -> Option<QubitRef> {
        if qubit == 0 {
            None
        } else {
            Some(QubitRef(qubit as u64))
        }
    }

    /// Converts the type-safe Rust representation of a qubit reference to the
    /// foreign representation.
    pub fn to_foreign(self) -> Result<u64> {
        if self.is_null() {
            inv_arg("making use of null qubit reference")
        } else {
            Ok(self.0 as u64)
        }
    }

    /// Converts the type-safe Rust representation of a qubit reference to the
    /// foreign representation.
    pub fn option_to_foreign(qubit: Option<QubitRef>) -> Result<u64> {
        if let Some(x) = qubit {
            x.to_foreign()
        } else {
            Ok(0)
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
    counter: std::ops::RangeFrom<u64>,
}

impl Default for QubitRefGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl QubitRefGenerator {
    /// Constructs a new reference generator.
    pub fn new() -> QubitRefGenerator {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_generator() {
        let mut q1 = QubitRefGenerator::default();
        let mut q2 = QubitRefGenerator::new();
        assert_eq!(q1.allocate(2), q2.allocate(2));
    }

    #[test]
    fn alloc_free() {
        let mut q = QubitRefGenerator::default();
        let qrefs = q.allocate(1);
        assert_eq!(qrefs.len(), 1);
        q.free(vec![QubitRef::from_foreign(4).unwrap()]);
        let qrefs = q.allocate(1);
        assert_eq!(qrefs[0], QubitRef::from_foreign(2).unwrap());
    }

    #[test]
    fn convert_qrefs() {
        let mut q = QubitRefGenerator::new();

        let qr = QubitRef::null();
        assert!(qr.to_foreign().is_err());
        assert!(QubitRef::option_to_foreign(Some(qr)).is_err());

        let qr = QubitRef(0);
        assert!(qr.to_foreign().is_err());
        assert!(QubitRef::option_to_foreign(Some(qr)).is_err());

        let qr = QubitRef::from_foreign(0);
        assert_eq!(qr, None);

        let qr = QubitRef::from_foreign(1).unwrap();
        assert_eq!(qr, (q.allocate(1))[0]);

        assert_eq!(
            42,
            QubitRef::from_foreign(42).unwrap().to_foreign().unwrap()
        );

        assert_eq!(0, QubitRef::option_to_foreign(None).unwrap());
        assert_eq!(
            42,
            QubitRef::option_to_foreign(Some(QubitRef::from_foreign(42).unwrap())).unwrap()
        );
    }

    #[test]
    fn display_qref() {
        let qubits = QubitRefGenerator::default().allocate(1);
        assert_eq!(qubits[0].to_string(), "1");
        let s: String = qubits[0].into();
        assert_eq!(s, "1".to_string());
    }
}
