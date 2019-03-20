use crate::error::{inv_arg, Error, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Represents a timeout parameter, which may be infinite.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Timeout {
    /// The duration specifies the maximum timeout.
    Duration(Duration),

    /// The timeout is infinite.
    Infinite,
}

impl Timeout {
    /// Creates a timeout from an integer time in seconds.
    pub fn from_seconds(seconds: u64) -> Timeout {
        Timeout::Duration(Duration::new(seconds, 0))
    }

    /// Creates a timeout from an integer time in milliseconds.
    pub fn from_millis(millis: u64) -> Timeout {
        Timeout::Duration(Duration::from_millis(millis))
    }

    /// Creates an infinite timeout.
    pub fn infinite() -> Timeout {
        Timeout::Infinite
    }

    /// Creates a timeout from an floating point time in seconds.
    ///
    /// Floating point infinity is used to specify infinite timeouts. This
    /// function fails when the specified time is negative.
    pub fn try_from_double(t: f64) -> Result<Timeout> {
        if t < 0.0 {
            inv_arg("timeouts cannot be negative")
        } else if t.is_infinite() {
            Ok(Timeout::infinite())
        } else {
            Ok(Timeout::Duration(Duration::from_nanos(
                (t * 1_000_000_000.0_f64) as u64,
            )))
        }
    }

    /// Returns the timeout as a floating point time in seconds.
    ///
    /// Floating point infinity is used to specify infinite timeouts.
    pub fn to_double(&self) -> f64 {
        if let Timeout::Duration(d) = self {
            (d.as_nanos() as f64) * 0.000_000_001_f64
        } else {
            std::f64::INFINITY
        }
    }

    /// Internal helper function for parsing duration strings.
    fn duration_from_component(num: &str, unit: &str) -> Result<Duration> {
        if let Ok(t) = num.parse::<u64>() {
            let unit = match unit {
                "h" => 3_600_000_000_000,
                "m" => 60_000_000_000,
                "s" => 1_000_000_000,
                "ms" => 1_000_000,
                "us" => 1_000,
                "ns" => 1,
                _ => inv_arg(format!(
                    "failed to parse timeout parameter: unknown time unit {}",
                    unit
                ))?,
            };
            Ok(Duration::from_nanos(t * unit))
        } else {
            inv_arg("failed to parse timeout parameter")
        }
    }
}

impl ::std::str::FromStr for Timeout {
    type Err = Error;

    /// Constructs a Timeout from its string representation.
    ///
    /// The string must either be "infinit[ey]" (or any case-insensitive
    /// substring thereof) or a time value, which consists of either a floating
    /// point number of seconds or one or more integers suffixed by "h" (hours),
    /// "m" (minutes), "s" (seconds), or "ms" (milliseconds) which are added
    /// together.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "" {
            return inv_arg("expected a timeout parameter");
        }
        let s = s.to_ascii_lowercase();

        // Try infinite option.
        if "infinite".starts_with(&s) || "infinity".starts_with(&s) {
            return Ok(Timeout::Infinite);
        }

        // Try to parse as a double.
        if let Ok(t) = s.parse::<f64>() {
            if t >= 0.0 {
                return Ok(Timeout::Duration(Duration::from_nanos(
                    (t * 1_000_000_000.0_f64) as u64,
                )));
            }
        }

        // Parse the hard way...
        let mut duration = Duration::new(0, 0);
        let mut num_start = 0;
        let mut unit_start = 0;
        let mut expect_num = true;
        let it = s.char_indices();
        for (i, c) in it {
            let is_num = c.is_numeric();
            if !is_num && !c.is_alphabetic() {
                return inv_arg("failed to parse timeout parameter");
            }
            if is_num != expect_num {
                if is_num {
                    if num_start == unit_start || unit_start == i {
                        return inv_arg("failed to parse timeout parameter");
                    }
                    duration += Timeout::duration_from_component(
                        &s[num_start..unit_start],
                        &s[unit_start..i],
                    )?;
                    num_start = i;
                } else {
                    unit_start = i;
                }
                expect_num = is_num;
            }
        }
        if expect_num {
            return inv_arg("failed to parse timeout parameter");
        }
        duration += Timeout::duration_from_component(&s[num_start..unit_start], &s[unit_start..])?;

        Ok(Timeout::Duration(duration))
    }
}

impl ::std::fmt::Display for Timeout {
    /// Turns the Timeout object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if let Timeout::Duration(_) = self {
            write!(f, "{}", self.to_double())
        } else {
            write!(f, "infinite")
        }
    }
}

#[cfg(test)]
mod test {

    use super::Timeout;
    use std::str::FromStr;
    use std::time::Duration;

    #[test]
    fn from_str() {
        assert_eq!(Timeout::from_str("infinite").unwrap(), Timeout::Infinite);
        assert_eq!(Timeout::from_str("infinity").unwrap(), Timeout::Infinite);
        assert_eq!(Timeout::from_str("INF").unwrap(), Timeout::Infinite);
        assert_eq!(Timeout::from_str("i").unwrap(), Timeout::Infinite);
        assert_eq!(
            Timeout::from_str("12").unwrap(),
            Timeout::Duration(Duration::new(12, 0))
        );
        assert_eq!(
            Timeout::from_str("12.3").unwrap(),
            Timeout::Duration(Duration::new(12, 300000000))
        );
        assert_eq!(
            Timeout::from_str("20s").unwrap(),
            Timeout::Duration(Duration::new(20, 0))
        );
        assert_eq!(
            Timeout::from_str("3m").unwrap(),
            Timeout::Duration(Duration::new(3 * 60, 0))
        );
        assert_eq!(
            Timeout::from_str("2h").unwrap(),
            Timeout::Duration(Duration::new(2 * 60 * 60, 0))
        );
        assert_eq!(
            Timeout::from_str("2h3m20s").unwrap(),
            Timeout::Duration(Duration::new(2 * 60 * 60 + 3 * 60 + 20, 0))
        );
        assert_eq!(
            Timeout::from_str("25ms").unwrap(),
            Timeout::Duration(Duration::new(0, 25000000))
        );
        assert_eq!(
            Timeout::from_str("25us").unwrap(),
            Timeout::Duration(Duration::new(0, 25000))
        );
        assert_eq!(
            Timeout::from_str("25ns").unwrap(),
            Timeout::Duration(Duration::new(0, 25))
        );
        assert_eq!(
            Timeout::from_str("2h3m20s100ms").unwrap(),
            Timeout::Duration(Duration::new(2 * 60 * 60 + 3 * 60 + 20, 100000000))
        );
        assert_eq!(
            Timeout::from_str("nope").unwrap_err().to_string(),
            "Invalid argument: failed to parse timeout parameter"
        );
        assert_eq!(
            Timeout::from_str("").unwrap_err().to_string(),
            "Invalid argument: expected a timeout parameter"
        );
        assert_eq!(Timeout::from_seconds(33).to_string(), "33");
        assert_eq!(Timeout::from_millis(42).to_string(), "0.042");
        assert_eq!(Timeout::Infinite.to_string(), "infinite");
    }
}
