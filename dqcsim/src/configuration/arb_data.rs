use failure::{Error, Fail};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Error structure used for reporting ArbData errors.
#[derive(Debug, Fail, PartialEq)]
pub enum ArbDataError {
    #[fail(display = "{}", 0)]
    ParseError(String),
}

/// Represents an ArbData structure, consisting of an (unparsed, TODO) JSON
/// string and a list of binary strings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArbData {
    pub json: serde_json::Value,
    pub args: Vec<Vec<u8>>,
}

impl ArbData {
    /// Scans and deserializes a JSON object from the given character iterator.
    ///
    /// *Only* the JSON object is taken from the iterator; that is, if there is
    /// additional data behind the JSON object, this data remains.
    fn scan_json_arg(it: &mut impl Iterator<Item = char>) -> Result<serde_json::Value, Error> {
        // First character must always be a {
        if it.next() != Some('{') {
            Err(ArbDataError::ParseError(
                "Expected JSON argument while parsing ArbData.".to_string(),
            ))?
        }

        // Loop over the rest of the JSON object.
        let mut json = "{".to_string();
        let mut obj_depth: usize = 1;
        let mut in_string = false;
        loop {
            if in_string {
                match it.next() {
                    Some('"') => {
                        // End of the string.
                        json.push('"');
                        in_string = false;
                    }
                    Some('\\') => {
                        // Scan escape sequence.
                        json.push('\\');
                        match it.next() {
                            Some('u') => {
                                json.push('u');
                                // \u takes four hex characters. We just assume
                                // that the chars are actually hex.
                                for _ in 0..4 {
                                    if let Some(c) = it.next() {
                                        json.push(c);
                                    } else {
                                        Err(ArbDataError::ParseError(
                                            "Unterminated JSON string while parsing ArbData."
                                                .to_string(),
                                        ))?;
                                    }
                                }
                            }
                            Some(c) => {
                                // Just assume that the escape sequence is OK.
                                json.push(c);
                            }
                            None => {
                                Err(ArbDataError::ParseError(
                                    "Unterminated JSON string while parsing ArbData.".to_string(),
                                ))?;
                            }
                        };
                    }
                    Some(c) => {
                        json.push(c);
                    }
                    None => {
                        Err(ArbDataError::ParseError(
                            "Unterminated JSON string while parsing ArbData.".to_string(),
                        ))?;
                    }
                };
            } else {
                match it.next() {
                    Some('{') => {
                        json.push('{');
                        obj_depth += 1;
                    }
                    Some('}') => {
                        json.push('}');
                        obj_depth -= 1;
                        if obj_depth == 0 {
                            // Finished scanning the JSON string. Now
                            // deserialize it.
                            match serde_json::from_str(&json) {
                                Ok(j) => return Ok(j),
                                Err(e) => Err(ArbDataError::ParseError(format!(
                                    "Error parsing JSON component of ArbData, {}: {}",
                                    json, e
                                )))?,
                            };
                        }
                    }
                    Some('"') => {
                        json.push('"');
                        in_string = true;
                    }
                    Some(c) => {
                        json.push(c);
                    }
                    None => {
                        Err(ArbDataError::ParseError(
                            "Unterminated JSON object while parsing ArbData.".to_string(),
                        ))?;
                    }
                };
            }
        }
    }

    /// Scans and deserializes a list of unstructured binary strings from the
    /// given character iterator.
    ///
    /// The unstructured binary strings are separated by commas. Furthermore,
    /// the following escape sequences are recognized:
    ///
    ///  - `_,` turns into a comma (`,`).
    ///  - `__` turns into an underscore (`_`).
    ///  - `_##` where ## is a 2-digit hexadecimal string turns into a byte
    ///    with the respective value.
    fn scan_unstructured_args(it: &mut impl Iterator<Item = char>) -> Result<Vec<Vec<u8>>, Error> {
        let mut output: Vec<Vec<u8>> = vec![];
        loop {
            let mut current: Vec<u8> = vec![];
            loop {
                match it.next() {
                    Some('_') => {
                        match it.next() {
                            Some('_') => {
                                current.push(b'_');
                            }
                            Some(',') => {
                                current.push(b',');
                            }
                            Some(c1) => {
                                if !c1.is_ascii_hexdigit() {
                                    Err(ArbDataError::ParseError("Invalid binary string escape sequence while parsing ArbData.".to_string()))?;
                                }
                                match it.next() {
                                    Some(c2) => {
                                        if !c2.is_ascii_hexdigit() {
                                            Err(ArbDataError::ParseError("Invalid binary string escape sequence while parsing ArbData.".to_string()))?;
                                        }
                                        let mut hex = c1.to_string();
                                        hex.push(c2);
                                        current.push(u8::from_str_radix(&hex, 16).unwrap());
                                    }
                                    None => {
                                        Err(ArbDataError::ParseError("Unterminated binary string escape sequence while parsing ArbData.".to_string()))?;
                                    }
                                }
                            }
                            None => {
                                Err(ArbDataError::ParseError("Unterminated binary string escape sequence while parsing ArbData.".to_string()))?;
                            }
                        }
                    }
                    Some(',') => {
                        break;
                    }
                    Some(c) => {
                        // oh my god rust why can't you just give me the code
                        // point...
                        let mut bytes = [0; 4];
                        for byte in c.encode_utf8(&mut bytes).bytes() {
                            current.push(byte);
                        }
                    }
                    None => {
                        output.push(current);
                        return Ok(output);
                    }
                }
            }
            output.push(current);
        }
    }

    /// Constructs an ArgData from a string containing only one or more
    /// unstructured binary arguments, using {} for the JSON object.
    ///
    /// The unstructured binary strings are separated by commas. Furthermore,
    /// the following escape sequences are recognized:
    ///
    ///  - `_,` turns into a comma (`,`).
    ///  - `__` turns into an underscore (`_`).
    ///  - `_##` where ## is a 2-digit hexadecimal string turns into a byte
    ///    with the respective value.
    ///
    /// To also parse a JSON object, use `from_str()`. To get an ArbData with
    /// the default JSON object and zero binary arguments, use `default()`.
    pub fn from_str_args_only(s: &str) -> Result<Self, Error> {
        Ok(ArbData {
            json: json!({}),
            args: ArbData::scan_unstructured_args(&mut s.chars())?,
        })
    }
}

impl ::std::str::FromStr for ArbData {
    type Err = Error;

    /// Constructs an ArgData from its string representation.
    ///
    /// The string starts with a JSON object. The object is followed by zero or
    /// more unstructured binary strings, which are separated by commas. The
    /// following escape sequences are recognized in the binary strings:
    ///
    ///  - `_,` turns into a comma (`,`).
    ///  - `__` turns into an underscore (`_`).
    ///  - `_##` where ## is a 2-digit hexadecimal string turns into a byte
    ///    with the respective value.
    ///
    /// To omit the JSON object and substitute the default {}, use
    /// `from_str_args_only()`. To get an ArbData with the default JSON object
    /// and zero binary arguments, use `default()`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = s.chars();
        let mut output = ArbData {
            json: ArbData::scan_json_arg(&mut iterator)?,
            args: vec![],
        };
        match iterator.next() {
            Some(',') => {
                output.args = ArbData::scan_unstructured_args(&mut iterator)?;
                Ok(output)
            }
            Some(c) => Err(ArbDataError::ParseError(format!(
                "Expected comma after JSON object in ArbData, received {}.",
                c
            )))?,
            None => Ok(output),
        }
    }
}

impl ::std::fmt::Display for ArbData {
    /// Turns the ArbData object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut output = self.json.to_string();
        for arg in self.args.iter() {
            output += ",";
            if arg.iter().any(|byte| *byte < 32 || *byte > 126) {
                for byte in arg.iter() {
                    output += &format!("_{:02X}", byte);
                }
            } else {
                for byte in arg.iter() {
                    match byte {
                        b'_' => output += "__",
                        b',' => output += "_,",
                        b => output.push(*b as char),
                    }
                }
            }
        }
        write!(f, "{}", output)
    }
}

impl Default for ArbData {
    /// Constructs an ArbData with JSON object {} and zero unstructured binary
    /// arguments.
    fn default() -> Self {
        ArbData {
            json: json!({}),
            args: vec![],
        }
    }
}

#[cfg(test)]
mod test {

    use super::{json, ArbData};
    use std::str::FromStr;

    fn test_from_str_good(input: &str, exp_json: serde_json::Value, exp_args: Vec<&[u8]>) {
        let exp_args = exp_args.into_iter().map(|x| x.to_vec()).collect();
        assert_eq!(
            ArbData::from_str(input).unwrap(),
            ArbData {
                json: exp_json,
                args: exp_args
            }
        );
    }

    fn test_from_str_fail(input: &str, msg: &str) {
        assert_eq!(ArbData::from_str(input).unwrap_err().to_string(), msg);
    }

    #[test]
    fn from_str() {
        test_from_str_good("{}", json!({}), vec![]);
        test_from_str_good("{},x,y,z", json!({}), vec![b"x", b"y", b"z"]);
        test_from_str_good(
            "{\"difficult\\u0020\\n\\t}\\\\\":33},x,y,z",
            json!({"difficult \n\t}\\": 33}),
            vec![b"x", b"y", b"z"],
        );
        test_from_str_fail(
            "{}}",
            "Expected comma after JSON object in ArbData, received }.",
        );
        test_from_str_fail("{{}", "Unterminated JSON object while parsing ArbData.");
        test_from_str_good("{},x_,y,z", json!({}), vec![b"x,y", b"z"]);
        test_from_str_good("{},_202_2f_,__,y,z", json!({}), vec![b" 2/,_", b"y", b"z"]);
        test_from_str_fail(
            "{},x,y,z_",
            "Unterminated binary string escape sequence while parsing ArbData.",
        );
        test_from_str_fail(
            "{},x,y,z_",
            "Unterminated binary string escape sequence while parsing ArbData.",
        );
    }

    fn test_to_str(json: serde_json::Value, args: Vec<&[u8]>, exp_output: &str) {
        let args = args.into_iter().map(|x| x.to_vec()).collect();
        let data = ArbData { json, args };
        let string = data.to_string();
        assert_eq!(string, exp_output);
        assert_eq!(ArbData::from_str(&string).unwrap(), data);
    }

    #[test]
    fn to_str() {
        test_to_str(json!({}), vec![], "{}");
        test_to_str(
            json!({"test": 42}),
            vec![b"x", b"y", b"z"],
            "{\"test\":42},x,y,z",
        );
        test_to_str(
            json!({}),
            vec![b"Hello, world!", b"\x01\x23\x45\x67\x89\xAB\xCD\xEF"],
            "{},Hello_, world!,_01_23_45_67_89_AB_CD_EF",
        );
    }
}
