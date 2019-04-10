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
    /// number. Every sequence number comes after `SequenceNumber::none()`.
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
