use crate::common::{
    error::{inv_arg, Error},
    types::{ArbCmd, ArbData},
    util::friendly_enum_parse,
};
use named_type::NamedType;
use named_type_derive::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Represents a host API call name.
#[derive(Display, NamedType, EnumIter, EnumString, Debug, Copy, Clone, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum HostCallFunction {
    Start,
    Wait,
    Send,
    Recv,
    Yield,
    Arb,
}

/// Represents a host API call.
///
/// This is used both by DQCsim itself to log API calls for outputting a
/// reproduction file when the host program requests it to, and by the command
/// line interface to specify the host API calls to be made.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum HostCall {
    Start(ArbData),
    Wait,
    Send(ArbData),
    Recv,
    Yield,
    Arb(String, ArbCmd),
}

impl ::std::str::FromStr for HostCall {
    type Err = Error;

    /// Constructs a HostCall from its string representation, which is one of:
    ///
    ///  - `start`
    ///  - `start:<ArbData>`
    ///  - `send:<ArbData>`
    ///  - `recv`
    ///  - `yield`
    ///  - `arb:<plugin>:<ArbCmd>`
    ///
    /// The function names may also be abbreviated.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the function name from its optional argument.
        let mut splitter = s.splitn(2, ':');
        let function = splitter.next().unwrap();
        let argument = splitter.next();
        assert_eq!(splitter.next(), None);

        // Parse the function name.
        let function: HostCallFunction = friendly_enum_parse(function)?;

        // Parse the argument based on the selected function and return.
        match argument {
            None => match function {
                HostCallFunction::Start => Ok(HostCall::Start(ArbData::default())),
                HostCallFunction::Wait => Ok(HostCall::Wait),
                HostCallFunction::Send => inv_arg("the send API call requires an ArbData argument"),
                HostCallFunction::Recv => Ok(HostCall::Recv),
                HostCallFunction::Yield => Ok(HostCall::Yield),
                HostCallFunction::Arb => {
                    inv_arg("the arb API call requires a plugin and an ArbCmd argument")
                }
            },
            Some(argument) => match function {
                HostCallFunction::Start => Ok(HostCall::Start(ArbData::from_str(argument)?)),
                HostCallFunction::Wait => inv_arg("the wait API call does not take an argument"),
                HostCallFunction::Send => Ok(HostCall::Send(ArbData::from_str(argument)?)),
                HostCallFunction::Recv => inv_arg("the recv API call does not take an argument"),
                HostCallFunction::Yield => inv_arg("the yield API call does not take an argument"),
                HostCallFunction::Arb => {
                    let mut splitter = argument.splitn(2, ':');
                    let arg1 = splitter.next().unwrap();
                    if let Some(arg2) = splitter.next() {
                        assert_eq!(splitter.next(), None);
                        Ok(HostCall::Arb(arg1.to_string(), ArbCmd::from_str(arg2)?))
                    } else {
                        inv_arg("the arb API call requires a plugin and an ArbCmd argument")
                    }
                }
            },
        }
    }
}

impl ::std::fmt::Display for HostCall {
    /// Turns the HostCall object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HostCall::Start(ref a) => write!(f, "start:{}", a),
            HostCall::Wait => write!(f, "wait"),
            HostCall::Send(ref a) => write!(f, "send:{}", a),
            HostCall::Recv => write!(f, "recv"),
            HostCall::Yield => write!(f, "yield"),
            HostCall::Arb(ref p, ref a) => write!(f, "arb:{}:{}", p, a),
        }
    }
}

#[cfg(test)]
mod test {

    use super::HostCall;
    use crate::common::types::{ArbCmd, ArbData};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        assert_eq!(
            HostCall::from_str("start").unwrap(),
            HostCall::Start(ArbData::default())
        );
        assert_eq!(
            HostCall::from_str("start:{\"answer\": 42},x,y,z").unwrap(),
            HostCall::Start(ArbData::from_str("{\"answer\": 42},x,y,z").unwrap())
        );
        assert_eq!(HostCall::from_str("wait").unwrap(), HostCall::Wait);
        assert_eq!(
            HostCall::from_str("wait:{\"answer\": 42},x,y,z")
                .unwrap_err()
                .to_string(),
            "Invalid argument: the wait API call does not take an argument"
        );
        assert_eq!(
            HostCall::from_str("send").unwrap_err().to_string(),
            "Invalid argument: the send API call requires an ArbData argument"
        );
        assert_eq!(
            HostCall::from_str("send:{\"answer\": 42},x,y,z").unwrap(),
            HostCall::Send(ArbData::from_str("{\"answer\": 42},x,y,z").unwrap())
        );
        assert_eq!(HostCall::from_str("recv").unwrap(), HostCall::Recv);
        assert_eq!(
            HostCall::from_str("recv:{\"answer\": 42},x,y,z")
                .unwrap_err()
                .to_string(),
            "Invalid argument: the recv API call does not take an argument"
        );
        assert_eq!(HostCall::from_str("yield").unwrap(), HostCall::Yield);
        assert_eq!(
            HostCall::from_str("yield:{\"answer\": 42},x,y,z")
                .unwrap_err()
                .to_string(),
            "Invalid argument: the yield API call does not take an argument"
        );
        assert_eq!(
            HostCall::from_str("arb").unwrap_err().to_string(),
            "Invalid argument: the arb API call requires a plugin and an ArbCmd argument"
        );
        assert_eq!(
            HostCall::from_str("arb:a").unwrap_err().to_string(),
            "Invalid argument: the arb API call requires a plugin and an ArbCmd argument"
        );
        assert_eq!(
            HostCall::from_str("arb:a:b.c:{\"answer\": 42},x,y,z").unwrap(),
            HostCall::Arb(
                "a".to_string(),
                ArbCmd::from_str("b.c:{\"answer\": 42},x,y,z").unwrap()
            )
        );
        assert_eq!(
            HostCall::from_str("hello").unwrap_err().to_string(),
            "Invalid argument: hello is not a valid host call function, valid values are start, wait, send, recv, yield, or arb"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", HostCall::Start(ArbData::default())),
            "start:{}"
        );
        assert_eq!(format!("{}", HostCall::Send(ArbData::default())), "send:{}");
        assert_eq!(
            format!(
                "{}",
                HostCall::Arb("a".to_string(), ArbCmd::new("a", "b", ArbData::default()))
            ),
            "arb:a:a.b:{}"
        );
        assert_eq!(format!("{}", HostCall::Wait), "wait");
        assert_eq!(format!("{}", HostCall::Recv), "recv");
        assert_eq!(format!("{}", HostCall::Yield), "yield");
    }

}
