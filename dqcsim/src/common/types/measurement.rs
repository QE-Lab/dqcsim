use crate::common::types::{ArbData, QubitRef};
use serde::{Deserialize, Serialize};

/// Represents the state of a single qubit measurement register.
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum QubitMeasurementState {
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

impl fmt::Display for QubitMeasurementState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QubitMeasurementState::Undefined => write!(f, "?"),
            QubitMeasurementState::Zero => write!(f, "0"),
            QubitMeasurementState::One => write!(f, "1"),
        }
    }
}

impl QubitMeasurementState {
    pub fn is_undefined(self) -> bool {
        self == QubitMeasurementState::Undefined
    }

    pub fn is_zero(self) -> bool {
        self == QubitMeasurementState::Zero
    }

    pub fn is_one(self) -> bool {
        self == QubitMeasurementState::One
    }
}

impl Into<Option<bool>> for QubitMeasurementState {
    fn into(self) -> Option<bool> {
        match self {
            QubitMeasurementState::Undefined => None,
            QubitMeasurementState::Zero => Some(false),
            QubitMeasurementState::One => Some(true),
        }
    }
}

impl From<Option<bool>> for QubitMeasurementState {
    fn from(source: Option<bool>) -> QubitMeasurementState {
        match source {
            None => QubitMeasurementState::Undefined,
            Some(false) => QubitMeasurementState::Zero,
            Some(true) => QubitMeasurementState::One,
        }
    }
}

impl From<bool> for QubitMeasurementState {
    fn from(source: bool) -> QubitMeasurementState {
        if source {
            QubitMeasurementState::One
        } else {
            QubitMeasurementState::Zero
        }
    }
}

/// Represents a qubit measurement result.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QubitMeasurementResult {
    /// The measured qubit.
    pub qubit: QubitRef,

    /// The measured value. true = 1, false = 0.
    pub value: QubitMeasurementState,

    /// Implementation-specific additional data, such as the probability for
    /// this particular measurement outcome.
    pub data: ArbData,
}

impl QubitMeasurementResult {
    pub fn new(
        qubit: impl Into<QubitRef>,
        value: impl Into<QubitMeasurementState>,
        data: impl Into<ArbData>,
    ) -> QubitMeasurementResult {
        QubitMeasurementResult {
            qubit: qubit.into(),
            value: value.into(),
            data: data.into(),
        }
    }
}
