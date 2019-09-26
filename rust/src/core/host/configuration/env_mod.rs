use serde::{Deserialize, Serialize};

/// Represents an environment variable modification.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum EnvMod {
    /// Sets the environment variable `key` to `value`.
    Set { key: String, value: String },
    /// Clears (undefines) the environment variable `key`.
    Remove { key: String },
}

impl EnvMod {
    /// Convenience method for building EnvMod::Set.
    pub fn set(key: impl Into<String>, value: impl Into<String>) -> EnvMod {
        EnvMod::Set {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Convenience method for building EnvMod::Remove.
    pub fn remove(key: impl Into<String>) -> EnvMod {
        EnvMod::Remove { key: key.into() }
    }
}

impl ::std::str::FromStr for EnvMod {
    type Err = failure::Error;

    /// Constructs an EnvMod from its string representation, which is one of:
    ///
    ///  - `<key>` - define environment variable `key` to the empty string.
    ///  - `<key>:<value>` - set environment variable `key` to `value`.
    ///  - `~<key>` - remove environment variable `key`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('~') {
            Ok(EnvMod::Remove {
                key: s[1..].to_string(),
            })
        } else {
            let mut splitter = s.splitn(2, '=');
            let key = splitter.next().unwrap().to_string();
            let value = splitter.next().unwrap_or("").to_string();
            assert_eq!(splitter.next(), None);
            Ok(EnvMod::Set { key, value })
        }
    }
}

impl ::std::fmt::Display for EnvMod {
    /// Turns the EnvMod object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EnvMod::Set { ref key, ref value } => write!(f, "{}={}", key, value),
            EnvMod::Remove { ref key } => write!(f, "~{}", key),
        }
    }
}

#[cfg(test)]
mod test {

    use super::EnvMod;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(EnvMod::from_str("a=b").unwrap(), EnvMod::set("a", "b"),);
        assert_eq!(EnvMod::from_str("a=b=c").unwrap(), EnvMod::set("a", "b=c"),);
        assert_eq!(EnvMod::from_str("FOO=").unwrap(), EnvMod::set("FOO", ""),);
        assert_eq!(EnvMod::from_str("bar").unwrap(), EnvMod::set("bar", ""),);
        assert_eq!(EnvMod::from_str("~baz").unwrap(), EnvMod::remove("baz"),);
    }

    #[test]
    fn to_str() {
        assert_eq!(EnvMod::set("a", "b").to_string(), "a=b",);
        assert_eq!(EnvMod::remove("a").to_string(), "~a",);
    }
}
