use crate::common::error::{inv_arg, Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

const EMPTY_CBOR: &[u8] = &[0xBF, 0xFF];

/// Represents an ArbData structure, consisting of an (unparsed, TODO) JSON
/// string and a list of binary strings.
#[derive(Clone, Hash, PartialEq, Deserialize, Serialize)]
pub struct ArbData {
    cbor: Vec<u8>,
    args: Vec<Vec<u8>>,
}

impl Eq for ArbData {}

impl fmt::Debug for ArbData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        /*let mut output: Vec<u8> = vec![];
        let mut de = serde_cbor::de::Deserializer::from_slice(&self.cbor);
        let mut ser = serde_json::Serializer::pretty(&mut output);
        serde_transcode::transcode(&mut de, &mut ser).unwrap();
        let output = String::from_utf8(output).unwrap();*/
        let value: serde_cbor::Value = serde_cbor::from_slice(&self.cbor).unwrap();

        fmt.debug_struct("ArbData")
            .field("json", &value)
            .field("args", &self.args)
            .finish()
    }
}

impl ArbData {
    /// Scans and deserializes a JSON object from the given character iterator.
    ///
    /// *Only* the JSON object is taken from the iterator; that is, if there is
    /// additional data behind the JSON object, this data remains.
    fn scan_json_arg(it: &mut impl Iterator<Item = char>) -> Result<Vec<u8>> {
        // First character must always be a {
        if it.next() != Some('{') {
            inv_arg("expected JSON argument while parsing ArbData")?
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
                                        inv_arg("unterminated JSON string while parsing ArbData")?;
                                    }
                                }
                            }
                            Some(c) => {
                                // Just assume that the escape sequence is OK.
                                json.push(c);
                            }
                            None => {
                                inv_arg("unterminated JSON string while parsing ArbData")?;
                            }
                        };
                    }
                    Some(c) => {
                        json.push(c);
                    }
                    None => {
                        inv_arg("unterminated JSON string while parsing ArbData")?;
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
                            // transmute it.
                            let mut cbor = vec![];
                            let mut de = serde_json::Deserializer::from_str(&json);
                            let mut ser = serde_cbor::ser::Serializer::new(&mut cbor);
                            match serde_transcode::transcode(&mut de, &mut ser) {
                                Err(e) => inv_arg(format!(
                                    "error parsing JSON component of ArbData, {}: {}",
                                    json, e
                                ))?,
                                _ => return Ok(cbor),
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
                        inv_arg("unterminated JSON object while parsing ArbData")?;
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
    fn scan_unstructured_args(it: &mut impl Iterator<Item = char>) -> Result<Vec<Vec<u8>>> {
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
                                    inv_arg("invalid binary string escape sequence while parsing ArbData")?;
                                }
                                match it.next() {
                                    Some(c2) => {
                                        if !c2.is_ascii_hexdigit() {
                                            inv_arg("invalid binary string escape sequence while parsing ArbData")?;
                                        }
                                        let mut hex = c1.to_string();
                                        hex.push(c2);
                                        current.push(u8::from_str_radix(&hex, 16).unwrap());
                                    }
                                    None => {
                                        inv_arg("unterminated binary string escape sequence while parsing ArbData")?;
                                    }
                                }
                            }
                            None => {
                                inv_arg("unterminated binary string escape sequence while parsing ArbData")?;
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
    pub fn from_str_args_only(s: &str) -> Result<Self> {
        Ok(ArbData {
            cbor: EMPTY_CBOR.to_owned(),
            args: ArbData::scan_unstructured_args(&mut s.chars())?,
        })
    }

    /// Construct an `ArbData` with just binary arguments and {} for the
    /// JSON/CBOR object.
    pub fn from_args(args: impl Into<Vec<Vec<u8>>>) -> Self {
        ArbData {
            cbor: EMPTY_CBOR.to_owned(),
            args: args.into(),
        }
    }

    /// Construct an `ArbData` from a CBOR object and binary arguments, without
    /// ensuring that the CBOR object is valid.
    pub fn from_cbor_unchecked(cbor: impl Into<Vec<u8>>, args: impl Into<Vec<Vec<u8>>>) -> Self {
        ArbData {
            cbor: cbor.into(),
            args: args.into(),
        }
    }

    /// Construct an `ArbData` from a CBOR object and binary arguments, while
    /// ensuring that the CBOR object is valid.
    pub fn from_cbor(cbor: impl AsRef<[u8]>, args: impl Into<Vec<Vec<u8>>>) -> Result<Self> {
        let mut arb_data = ArbData {
            cbor: vec![],
            args: args.into(),
        };
        arb_data.set_cbor(cbor)?;
        Ok(arb_data)
    }

    /// Construct an `ArbData` from a JSON object and binary arguments, while
    /// ensuring that the JSON object is valid.
    pub fn from_json(json: impl AsRef<str>, args: impl Into<Vec<Vec<u8>>>) -> Result<Self> {
        let mut arb_data = ArbData {
            cbor: vec![],
            args: args.into(),
        };
        arb_data.set_json(json)?;
        Ok(arb_data)
    }

    /// Returns the JSON/CBOR data field as a JSON string.
    pub fn get_json(&self) -> Result<String> {
        let mut output: Vec<u8> = vec![];
        let mut de = serde_cbor::de::Deserializer::from_slice(&self.cbor);
        let mut ser = serde_json::Serializer::new(&mut output);
        serde_transcode::transcode(&mut de, &mut ser)?;
        Ok(String::from_utf8(output)?)
    }

    /// Returns the JSON/CBOR data field as a CBOR string.
    pub fn get_cbor(&self) -> &[u8] {
        &self.cbor
    }

    /// Provides a reference to the binary argument vector.
    pub fn get_args(&self) -> &[Vec<u8>] {
        &self.args
    }

    /// Provides a mutable reference to the binary argument vector.
    pub fn get_args_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.args
    }

    /// Sets the JSON/CBOR data field by means of a JSON string.
    pub fn set_json(&mut self, json: impl AsRef<str>) -> Result<()> {
        let mut output: Vec<u8> = vec![];
        let mut de = serde_json::Deserializer::from_str(json.as_ref());
        let mut ser = serde_cbor::ser::Serializer::new(&mut output);
        if let Err(e) = serde_transcode::transcode(&mut de, &mut ser) {
            inv_arg(e.to_string())
        } else {
            self.cbor = output;
            Ok(())
        }
    }

    /// Sets the JSON/CBOR data field by means of a CBOR string.
    pub fn set_cbor(&mut self, cbor: impl AsRef<[u8]>) -> Result<()> {
        let mut output: Vec<u8> = vec![];
        let mut de = serde_cbor::de::Deserializer::from_slice(cbor.as_ref());
        let mut ser = serde_cbor::ser::Serializer::new(&mut output);
        if let Err(e) = serde_transcode::transcode(&mut de, &mut ser) {
            inv_arg(e.to_string())
        } else {
            self.cbor = output;
            Ok(())
        }
    }

    /// Sets the JSON/CBOR data field by means of a CBOR string without
    /// ensuring that the CBOR is valid.
    pub fn set_cbor_unchecked(&mut self, cbor: impl Into<Vec<u8>>) {
        self.cbor = cbor.into();
    }

    /// Provides a reference to the binary argument vector.
    pub fn set_args(&mut self, args: impl Into<Vec<Vec<u8>>>) {
        self.args = args.into();
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
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut iterator = s.chars();
        let mut output = ArbData {
            cbor: ArbData::scan_json_arg(&mut iterator)?,
            args: vec![],
        };
        match iterator.next() {
            Some(',') => {
                output.args = ArbData::scan_unstructured_args(&mut iterator)?;
                Ok(output)
            }
            Some(c) => inv_arg(format!(
                "expected comma after JSON object in ArbData, received {}",
                c
            )),
            None => Ok(output),
        }
    }
}

impl ::std::fmt::Display for ArbData {
    /// Turns the ArbData object into a string representation that can be
    /// parsed by `from_str()`.
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut output = self.get_json().map_err(|_| std::fmt::Error)?;
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
            cbor: EMPTY_CBOR.to_owned(),
            args: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::ArbData;
    use serde_json::json;
    use std::str::FromStr;

    fn test_from_str_good(input: &str, exp_json: serde_json::Value, exp_args: Vec<&[u8]>) {
        let actual = ArbData::from_str(input).unwrap();
        let exp_args: Vec<Vec<u8>> = exp_args.into_iter().map(|x| x.to_vec()).collect();
        assert_eq!(actual.get_args(), &exp_args[..]);
        assert_eq!(actual.get_json().unwrap(), exp_json.to_string());
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
            "Invalid argument: expected comma after JSON object in ArbData, received }",
        );
        test_from_str_fail(
            "{{}",
            "Invalid argument: unterminated JSON object while parsing ArbData",
        );
        test_from_str_good("{},x_,y,z", json!({}), vec![b"x,y", b"z"]);
        test_from_str_good("{},_202_2f_,__,y,z", json!({}), vec![b" 2/,_", b"y", b"z"]);
        test_from_str_fail(
            "{},x,y,z_",
            "Invalid argument: unterminated binary string escape sequence while parsing ArbData",
        );
        test_from_str_fail(
            "{},x,y,z_",
            "Invalid argument: unterminated binary string escape sequence while parsing ArbData",
        );
    }

    fn test_to_str(json: serde_json::Value, args: Vec<&[u8]>, exp_output: &str) {
        let args: Vec<Vec<u8>> = args.into_iter().map(|x| x.to_vec()).collect();
        let data = ArbData::from_json(json.to_string(), args).unwrap();
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

    #[test]
    fn eq() {
        let args: Vec<Vec<u8>> = vec![b"x", b"y", b"z"]
            .into_iter()
            .map(|x| x.to_vec())
            .collect();
        let data = ArbData::from_json(json!({"test": 42}).to_string(), args).unwrap();
        assert_eq!(data, data);
        assert_eq!(ArbData::default(), ArbData::default());
    }
}
