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
