use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a sequence number used within a gate stream.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SequenceNumber(u64);

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SequenceNumber {
    /// Returns true if receiving this sequence number in a `Success` message
    /// acknowledges the given other sequence number.
    pub fn acknowledges(self, other: SequenceNumber) -> bool {
        self.0 >= other.0
    }

    /// Returns true if this sequence number comes after the given sequence
    /// number. Every sequence number comes after `SequenceNumber::none()`,
    /// except for `SequenceNumber::none()` which never comes after anything.
    pub fn after(self, other: SequenceNumber) -> bool {
        self.0 > other.0
    }

    /// Returns the sequence number that comes before this one.
    pub fn preceding(self) -> SequenceNumber {
        if self.0 == 0 {
            SequenceNumber(0)
        } else {
            SequenceNumber(self.0 - 1)
        }
    }

    /// "None" value for sequence numbers, used to indicate that nothing has
    /// been transferred yet.
    pub fn none() -> SequenceNumber {
        SequenceNumber(0)
    }
}

/// Struct used to generate sequence numbers.
///
/// Sequence numbers start at 0 and count up monotonously.
pub struct SequenceNumberGenerator {
    counter: std::ops::RangeFrom<u64>,
    previous: SequenceNumber,
}

impl Default for SequenceNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SequenceNumberGenerator {
    /// Constructs a new sequence number generator.
    pub fn new() -> SequenceNumberGenerator {
        SequenceNumberGenerator {
            counter: (1..),
            previous: SequenceNumber::none(),
        }
    }

    /// Acquires the next sequence number.
    pub fn get_next(&mut self) -> SequenceNumber {
        let next = SequenceNumber((&mut self.counter).next().unwrap());
        self.previous = next;
        next
    }

    /// Returns the previously acquired sequence number.
    pub fn get_previous(&self) -> SequenceNumber {
        self.previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_generator() {
        let mut s1 = SequenceNumberGenerator::default();
        let mut s2 = SequenceNumberGenerator::new();
        assert_eq!(s1.get_next(), s2.get_next());
    }

    #[test]
    fn gets() {
        let mut s = SequenceNumberGenerator::default();
        assert_eq!(s.get_previous(), SequenceNumber(0));
        assert_eq!(s.get_previous(), SequenceNumber(0));
        assert_eq!(s.get_next(), SequenceNumber(1));
        assert_eq!(s.get_next(), SequenceNumber(2));
        assert_eq!(s.get_previous(), SequenceNumber(2));
        assert_eq!(s.get_next(), SequenceNumber(3));
        assert_eq!(s.get_previous(), SequenceNumber(3));
    }

    #[test]
    fn seqs() {
        let mut s = SequenceNumberGenerator::default();
        let sn = s.get_previous();

        assert_eq!(sn.preceding(), SequenceNumber(0));
        assert!(!sn.after(SequenceNumber(0)));
        assert!(!sn.after(SequenceNumber(1)));
        assert!(sn.acknowledges(SequenceNumber(0)));
        assert!(!sn.acknowledges(SequenceNumber(1)));

        let sn = s.get_next();
        assert_eq!(sn.preceding(), SequenceNumber(0));
        assert!(sn.after(SequenceNumber(0)));
        assert!(!sn.after(SequenceNumber(1)));
        assert!(sn.acknowledges(SequenceNumber(0)));
        assert!(sn.acknowledges(SequenceNumber(1)));

        let sn = s.get_next();
        assert_eq!(sn.preceding(), SequenceNumber(1));
        assert!(sn.after(SequenceNumber(0)));
        assert!(sn.after(SequenceNumber(1)));
        assert!(!sn.after(SequenceNumber(2)));
        assert!(sn.acknowledges(SequenceNumber(0)));
        assert!(sn.acknowledges(SequenceNumber(1)));
        assert!(sn.acknowledges(SequenceNumber(2)));
        assert!(!sn.acknowledges(SequenceNumber(3)));
    }

    #[test]
    fn display() {
        assert_eq!(SequenceNumber(123).to_string(), "123");
    }
}
