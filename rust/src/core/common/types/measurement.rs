use crate::common::types::{ArbData, QubitRef};
use serde::{Deserialize, Serialize};

/// Represents the state of a single qubit measurement register.
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum QubitMeasurementValue {
    /// The value is unknown because the qubit has not been measured yet, or
    /// the most recent measurement failed.
    ///
    /// DQCsim also sets qubit measurements to undefined when it receives
    /// unexpected measurement results, no measurement result when one was
    /// expected, or multiple measurement results when one or none were
    /// expected.
    Undefined,

    /// The qubit was measured to be zero.
    Zero,

    /// The qubit was measured to be one.
    One,
}

impl fmt::Display for QubitMeasurementValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QubitMeasurementValue::Undefined => write!(f, "?"),
            QubitMeasurementValue::Zero => write!(f, "0"),
            QubitMeasurementValue::One => write!(f, "1"),
        }
    }
}

impl QubitMeasurementValue {
    pub fn is_undefined(self) -> bool {
        self == QubitMeasurementValue::Undefined
    }

    pub fn is_zero(self) -> bool {
        self == QubitMeasurementValue::Zero
    }

    pub fn is_one(self) -> bool {
        self == QubitMeasurementValue::One
    }
}

impl Into<Option<bool>> for QubitMeasurementValue {
    fn into(self) -> Option<bool> {
        match self {
            QubitMeasurementValue::Undefined => None,
            QubitMeasurementValue::Zero => Some(false),
            QubitMeasurementValue::One => Some(true),
        }
    }
}

impl From<Option<bool>> for QubitMeasurementValue {
    fn from(source: Option<bool>) -> QubitMeasurementValue {
        match source {
            None => QubitMeasurementValue::Undefined,
            Some(false) => QubitMeasurementValue::Zero,
            Some(true) => QubitMeasurementValue::One,
        }
    }
}

impl From<bool> for QubitMeasurementValue {
    fn from(source: bool) -> QubitMeasurementValue {
        if source {
            QubitMeasurementValue::One
        } else {
            QubitMeasurementValue::Zero
        }
    }
}

/// Represents a qubit measurement result.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QubitMeasurementResult {
    /// The measured qubit.
    pub qubit: QubitRef,

    /// The measured value.
    pub value: QubitMeasurementValue,

    /// Implementation-specific additional data, such as the probability for
    /// this particular measurement outcome.
    pub data: ArbData,
}

impl QubitMeasurementResult {
    pub fn new(
        qubit: impl Into<QubitRef>,
        value: impl Into<QubitMeasurementValue>,
        data: impl Into<ArbData>,
    ) -> QubitMeasurementResult {
        QubitMeasurementResult {
            qubit: qubit.into(),
            value: value.into(),
            data: data.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let m = QubitMeasurementValue::Undefined;
        assert_eq!(m.to_string(), "?");
        let m = QubitMeasurementValue::Zero;
        assert_eq!(m.to_string(), "0");
        let m = QubitMeasurementValue::One;
        assert_eq!(m.to_string(), "1");
    }

    #[test]
    fn is_funcs() {
        let m = QubitMeasurementValue::Undefined;
        assert!(m.is_undefined());
        let m = QubitMeasurementValue::Zero;
        assert!(m.is_zero());
        let m = QubitMeasurementValue::One;
        assert!(m.is_one());
    }

    #[test]
    fn convert() {
        let m = QubitMeasurementValue::Undefined;
        let o: Option<bool> = m.into();
        assert_eq!(o, None);
        let t: QubitMeasurementValue = o.into();
        assert!(t.is_undefined());

        let m = QubitMeasurementValue::One;
        let o: Option<bool> = m.into();
        assert_eq!(o, Some(true));
        let t: QubitMeasurementValue = o.into();
        assert!(t.is_one());

        let m = QubitMeasurementValue::Zero;
        let o: Option<bool> = m.into();
        assert_eq!(o, Some(false));
        let t: QubitMeasurementValue = o.into();
        assert!(t.is_zero());

        let z = false;
        let m: QubitMeasurementValue = z.into();
        assert!(m.is_zero());

        let o = true;
        let m: QubitMeasurementValue = o.into();
        assert!(m.is_one());
    }
}
