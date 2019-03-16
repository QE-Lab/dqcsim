use failure::{Error, Fail};
use serde::{Deserialize, Serialize};

use crate::configuration::arb_data::ArbData;

/// Error structure used for reporting ArbCmd errors.
#[derive(Debug, Fail, PartialEq)]
pub enum ArbCmdError {
    #[fail(display = "{}", 0)]
    ParseError(String),
    #[fail(display = "{}", 0)]
    IdError(String),
}

/// Represents an ArbCmd structure, consisting of interface and operation
/// identifier strings and an ArbData object.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArbCmd {
    interface_identifier: String,
    operation_identifier: String,
    data: ArbData,
}

impl ArbCmd {
    /// Verifies that the given identifier does not contain invalid characters.
    fn verify_id(id: String) -> Result<String, Error> {
        if id.chars().any(|x| !(x.is_ascii_alphanumeric() || x == '_')) {
            Err(ArbCmdError::IdError(format!(
                "\"{}\" is not a valid identifier; it contains characters outside [a-zA-Z0-9_].",
                id
            )))?
        } else {
            Ok(id)
        }
    }

    /// Constructs an ArbCmd.
    ///
    /// The identifiers may only contain characters in the pattern
    /// `[a-zA-Z0-9_]`. It this is not the case, this function panics.
    pub fn new(
        interface_identifier: impl Into<String>,
        operation_identifier: impl Into<String>,
        data: ArbData,
    ) -> ArbCmd {
        ArbCmd {
            interface_identifier: ArbCmd::verify_id(interface_identifier.into()).unwrap(),
            operation_identifier: ArbCmd::verify_id(operation_identifier.into()).unwrap(),
            data,
        }
    }

    /// Returns a reference to the interface identifier for this ArbCmd.
    pub fn interface_identifier(&self) -> &str {
        &self.interface_identifier
    }

    /// Returns a reference to the operation identifier for this ArbCmd.
    pub fn operation_identifier(&self) -> &str {
        &self.operation_identifier
    }

    /// Returns a reference to the data for this ArbCmd.
    pub fn data(&self) -> &ArbData {
        &self.data
    }

    /// Returns a mutable reference to the data for this ArbCmd.
    pub fn data_mut(&mut self) -> &mut ArbData {
        &mut self.data
    }
}

impl ::std::str::FromStr for ArbCmd {
    type Err = Error;

    /// Constructs an ArbCmd from its string representation. The following
    /// representations are allowed:
    ///
    ///  - `<interface-id>.<operation-id>` (use `ArbData::default()`)
    ///  - `<interface-id>.<operation-id>:<json>,<arg1>,<arg2>,[...]`
    ///    (use `ArbData::from_str()`)
    ///  - `<interface-id>.<operation-id>.<arg1>,<arg2>,[...]` (use
    ///    `ArbData::from_str_args_only()`)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split off and validate the interface identifier.
        let mut x = s.splitn(2, '.');
        let interface_identifier = ArbCmd::verify_id(x.next().unwrap().to_string())?;
        let s = x.next().ok_or_else(|| {
            ArbCmdError::ParseError(
                "Expected period after interface identifier while parsing ArbCmd.".to_string(),
            )
        })?;
        assert_eq!(x.next(), None);

        // Figure out where and how to split the operation identifier from the
        // data argument.
        let offs_period = s.find('.');
        let offs_semicolon = s.find(':');
        enum ArgMode {
            Omitted,
            JsonOmited(usize),
            Complete(usize),
        };
        let mode = match offs_period {
            Some(offs_period) => match offs_semicolon {
                Some(offs_semicolon) => {
                    if offs_period < offs_semicolon {
                        ArgMode::JsonOmited(offs_period)
                    } else {
                        ArgMode::Complete(offs_semicolon)
                    }
                }
                None => ArgMode::JsonOmited(offs_period),
            },
            None => match offs_semicolon {
                Some(offs_semicolon) => ArgMode::Complete(offs_semicolon),
                None => ArgMode::Omitted,
            },
        };

        // Split off and validate the operation identifier and parse the ArbCmd
        // in the way we just detected.
        match mode {
            ArgMode::Omitted => Ok(ArbCmd {
                interface_identifier,
                operation_identifier: ArbCmd::verify_id(s.to_string())?,
                data: ArbData::default(),
            }),
            ArgMode::JsonOmited(offs) => Ok(ArbCmd {
                interface_identifier,
                operation_identifier: ArbCmd::verify_id(s[..offs].to_string())?,
                data: ArbData::from_str_args_only(&s[offs + 1..])?,
            }),
            ArgMode::Complete(offs) => Ok(ArbCmd {
                interface_identifier,
                operation_identifier: ArbCmd::verify_id(s[..offs].to_string())?,
                data: ArbData::from_str(&s[offs + 1..])?,
            }),
        }
    }
}

impl ::std::fmt::Display for ArbCmd {
    /// Turns the ArbCmd object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}.{}:{}",
            self.interface_identifier, self.operation_identifier, self.data
        )
    }
}

#[cfg(test)]
mod test {

    use super::ArbCmd;
    use crate::configuration::arb_data::ArbData;
    use serde_json::json;
    use std::str::FromStr;

    fn test_from_str_good(
        input: &str,
        exp_iface: &str,
        exp_oper: &str,
        exp_json: serde_json::Value,
        exp_args: Vec<&[u8]>,
    ) {
        let exp_args = exp_args.into_iter().map(|x| x.to_vec()).collect();
        assert_eq!(
            ArbCmd::from_str(input).unwrap(),
            ArbCmd::new(
                exp_iface,
                exp_oper,
                ArbData {
                    json: exp_json,
                    args: exp_args
                }
            )
        );
    }

    fn test_from_str_fail(input: &str, msg: &str) {
        assert_eq!(ArbCmd::from_str(input).unwrap_err().to_string(), msg);
    }

    #[test]
    fn from_str() {
        test_from_str_good("a.b", "a", "b", json!({}), vec![]);
        test_from_str_good("a.b.x,y,z", "a", "b", json!({}), vec![b"x", b"y", b"z"]);
        test_from_str_good(
            "a.b:{\"answer\":42}",
            "a",
            "b",
            json!({"answer": 42}),
            vec![],
        );
        test_from_str_good(
            "a.b:{\"answer\":42},x,y,z",
            "a",
            "b",
            json!({"answer": 42}),
            vec![b"x", b"y", b"z"],
        );
        test_from_str_good("a.b.:x,y,z", "a", "b", json!({}), vec![b":x", b"y", b"z"]);
        test_from_str_good(
            "a.b:{\"answer\":42},.x,y,z",
            "a",
            "b",
            json!({"answer": 42}),
            vec![b".x", b"y", b"z"],
        );
        test_from_str_fail(
            "a",
            "Expected period after interface identifier while parsing ArbCmd.",
        );
        test_from_str_fail(
            "a%.b",
            "\"a%\" is not a valid identifier; it contains characters outside [a-zA-Z0-9_].",
        );
        test_from_str_fail(
            "a.b%",
            "\"b%\" is not a valid identifier; it contains characters outside [a-zA-Z0-9_].",
        );
    }

    fn test_to_str(
        iface: &str,
        oper: &str,
        json: serde_json::Value,
        args: Vec<&[u8]>,
        exp_output: &str,
    ) {
        let args = args.into_iter().map(|x| x.to_vec()).collect();
        let cmd = ArbCmd::new(iface, oper, ArbData { json, args });
        let string = cmd.to_string();
        assert_eq!(string, exp_output);
        assert_eq!(ArbCmd::from_str(&string).unwrap(), cmd);
    }

    #[test]
    fn to_str() {
        test_to_str("a", "b", json!({}), vec![], "a.b:{}");
        test_to_str(
            "a",
            "b",
            json!({"answer": 42}),
            vec![b"x", b"y", b"z"],
            "a.b:{\"answer\":42},x,y,z",
        );
    }
}
