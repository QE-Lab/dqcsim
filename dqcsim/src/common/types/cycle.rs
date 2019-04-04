use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

/// Represents a cycle difference.
pub type CycleDelta = i64;

/// Represents a number of cycles to advance by.
pub type Cycles = u64;

/// Represents a simulation cycle.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cycle(i64);

impl fmt::Display for Cycle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Advance by N cycles.
impl Add<CycleDelta> for Cycle {
    type Output = Cycle;

    fn add(self, other: CycleDelta) -> Cycle {
        let c = self.0.checked_add(other).expect("Cycle count overflow");
        assert!(c >= 0, "Cycle count overflow");
        Cycle(c)
    }
}

// Revert by N cycles.
impl Sub<CycleDelta> for Cycle {
    type Output = Cycle;

    fn sub(self, other: CycleDelta) -> Cycle {
        let c = self.0.checked_sub(other).expect("Cycle count overflow");
        assert!(c >= 0, "Cycle count overflow");
        Cycle(c)
    }
}

// Compute cycle difference.
impl Sub for Cycle {
    type Output = CycleDelta;

    fn sub(self, other: Cycle) -> CycleDelta {
        self.0.checked_sub(other.0).expect("Cycle count overflow")
    }
}

impl Cycle {
    /// Returns the first simulation cycle.
    pub fn t_zero() -> Cycle {
        Cycle(0)
    }

    /// Advances by the specified number of cycles.
    pub fn advance(self, cycles: Cycles) -> Cycle {
        assert!(cycles <= (std::i64::MAX as u64), "Cycle count overflow");
        Cycle(
            self.0
                .checked_add(cycles as i64)
                .expect("Cycle count overflow"),
        )
    }
}

impl Into<u64> for Cycle {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl Into<i64> for Cycle {
    fn into(self) -> i64 {
        self.0
    }
}
