use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a 64-bit random seed. Can be converted to and from a string as
/// you would expect a seed to be. Also implements std::default::Default,
/// which returns a random seed (from the random crate).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Seed {
    /// The random seed.
    pub value: u64,
}

impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl From<&str> for Seed {
    /// Turns a string into a seed. The string is parsed as a u64 if possible;
    /// if this fails, Rust's default hasher is used to turn the string into a
    /// u64.
    fn from(s: &str) -> Self {
        if let Ok(seed) = s.parse::<u64>() {
            Seed { value: seed }
        } else {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            s.hash(&mut hasher);
            Seed {
                value: hasher.finish(),
            }
        }
    }
}

impl Into<u64> for Seed {
    /// Returns the embedded seed.
    fn into(self) -> u64 {
        self.value
    }
}

impl Default for Seed {
    /// Returns a randomly generated seed using the random crate.
    fn default() -> Self {
        Seed {
            value: rand::random(),
        }
    }
}
