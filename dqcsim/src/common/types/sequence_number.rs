use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a sequence number used within a gate stream.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SequenceNumber(usize);

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
}

/// Struct used to generate sequence numbers.
///
/// Sequence numbers start at 0 and count up monotonously.
pub struct SequenceNumberGenerator {
    counter: std::ops::RangeFrom<usize>,
}

impl Default for SequenceNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SequenceNumberGenerator {
    /// Constructs a new sequence number generator.
    pub fn new() -> SequenceNumberGenerator {
        SequenceNumberGenerator { counter: (0..) }
    }

    /// Acquires the next sequence number.
    pub fn get_next(&mut self) -> SequenceNumber {
        SequenceNumber((&mut self.counter).next().unwrap())
    }
}
