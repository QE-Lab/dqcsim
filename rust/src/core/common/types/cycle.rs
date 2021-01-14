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
        assert!(self.0 >= 0, "Cycle count negative");
        self.0 as u64
    }
}

impl Into<i64> for Cycle {
    fn into(self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let c = Cycle::t_zero();
        assert_eq!(c, Cycle(0i64));
    }

    #[test]
    fn advance() {
        let c = Cycle::t_zero();
        let c = c.advance(1234u64);
        assert_eq!(c, Cycle(1234));
        let c = c.advance(1u64);
        assert_eq!(c, Cycle(1235));
    }

    #[test]
    #[should_panic]
    fn overflow() {
        let c = Cycle::t_zero();
        c.advance(std::u64::MAX);
    }

    #[test]
    #[should_panic]
    fn convert_negative() {
        let a = 18_446_744_073_709_551_574u64;
        let c = Cycle(-42);
        let c: u64 = c.into();
        assert_eq!(a, c);
    }

    #[test]
    fn convert() {
        let a = 1234u64;
        let c = Cycle(1234);
        let c: u64 = c.into();
        assert_eq!(a, c);

        let a = 1234i64;
        let c = Cycle(1234);
        let c: i64 = c.into();
        assert_eq!(a, c);
    }

    #[allow(clippy::eq_op)]
    #[test]
    fn ops() {
        let a = Cycle(21);
        let b: CycleDelta = 21;
        let c: CycleDelta = -2;
        assert_eq!(a + b, Cycle(42));
        assert_eq!(a - b, Cycle(0));
        assert_eq!(a + c, Cycle(19));
        assert_eq!(a - c, Cycle(23));
        assert_eq!(a - a, 0_i64);
    }
}
