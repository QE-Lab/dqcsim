use crate::common::error::{inv_arg, Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

mod cbor_canon {
    use crate::common::error::{inv_arg, oe_inv_arg, Result};

    /// Converts a slice of u8's to an u64 using CBOR byte order.
    fn bytes_as_u64(input: &[u8], num: usize) -> Result<u64> {
        let mut value = 0;
        for i in 0..num {
            value <<= 8;
            value |= *input
                .get(i)
                .ok_or_else(oe_inv_arg("invalid CBOR: expected additional tag byte"))?
                as u64;
        }
        Ok(value)
    }

    /// Inverse of bytes_as_u64.
    fn u64_as_bytes(input: u64, num: usize, output: &mut Vec<u8>) {
        for i in 0..num {
            output.push((input >> ((num - i - 1) * 8)) as u8);
        }
    }

    /// Reads the tag bytes for a CBOR value. Returns the major type, the
    /// additional information value (or `None` to encode an indefinite length),
    /// and the number of bytes read for the tag. Note: 0xFF is not considered
    /// a valid tag; check for this first if you're expecting it.
    fn read_tag(input: &[u8]) -> Result<(u8, u8, Option<u64>, usize)> {
        // Read initial byte.
        let initial: u8 = *input
            .get(0)
            .ok_or_else(oe_inv_arg("invalid CBOR: expected tag"))?;
        if initial == 0xFF {
            inv_arg("invalid CBOR: unexpected break")?;
        }
        let major = initial >> 5;
        let minor = initial & 0x1F;

        // Parse additional information.
        let (additional, index): (Option<u64>, usize) = match minor {
            0..=23 => (Some(minor as u64), 1),
            24 => (Some(bytes_as_u64(&input[1..], 1)?), 2),
            25 => (Some(bytes_as_u64(&input[1..], 2)?), 3),
            26 => (Some(bytes_as_u64(&input[1..], 4)?), 5),
            27 => (Some(bytes_as_u64(&input[1..], 8)?), 9),
            28..=30 => inv_arg("invalid CBOR: reserved minor tag value")?,
            31 => (None, 1),
            _ => unreachable!(),
        };

        Ok((major, minor, additional, index))
    }

    /// Canonicalizes the value at the front of the given cbor string. Output is
    /// pushed to the back of `output`. Returns the number of input bytes consumed.
    fn canonicalize_int(input: &[u8], output: &mut Vec<u8>) -> Result<usize> {
        //eprintln!("ENTER canonicalize_int({:?}, {:?})", input, output);

        // Read the tag.
        let (major, minor, additional, tag_len) = read_tag(input)?;
        //eprintln!("      major={:?}, minor={:?}, additional={:?})", major, minor, additional);
        let input = &input[tag_len..];

        // Parse contents.
        let (content, input_len, additional) =
            match major {
                0 | 1 | 7 => {
                    // nothing to do for types without content
                    (
                        vec![],
                        0,
                        additional.ok_or_else(oe_inv_arg(
                            "invalid CBOR: indefinite length tag for non-sequence",
                        ))?,
                    )
                }
                2 | 3 => {
                    if let Some(content_len) = additional {
                        // simple bytes/UTF8: additional value is number of bytes, just
                        // copy them
                        let content_size = content_len as usize;
                        (
                            input
                                .get(..content_size)
                                .ok_or_else(oe_inv_arg("invalid CBOR: incomplete bytes/utf8"))?
                                .to_vec(),
                            content_size,
                            content_len,
                        )
                    } else {
                        // chunked bytes/UTF8: concatenate the contents of the chunks
                        // and count the number of bytes, to be stored in the definite
                        // length tag
                        let mut index = 0;
                        let mut content = vec![];
                        while *input
                            .get(index)
                            .ok_or_else(oe_inv_arg("invalid CBOR: incomplete chunked bytes/utf8"))?
                            != 0xFF
                        {
                            let (chunk_major, _, chunk_len, chunk_tag_len) = read_tag(input)?;

                            // chunk types must equal the outer type
                            if chunk_major != major {
                                inv_arg("invalid CBOR: illegal non-bytes/utf8 chunk")?;
                            }

                            // chunk lengths must be definite
                            let chunk_len = chunk_len
                                .ok_or_else(oe_inv_arg("invalid CBOR: indefinite chunk size"))?;

                            // skip past the chunk tag
                            index += chunk_tag_len as usize;

                            // copy the chunk data into the output
                            content.extend_from_slice(
                                input.get(index..index + chunk_len as usize).ok_or_else(
                                    oe_inv_arg("invalid CBOR: incomplete bytes/utf8 chunk"),
                                )?,
                            );
                            index += chunk_len as usize;
                        }
                        index += 1; // for the 0xFF
                        let additional = content.len() as u64;
                        (content, index, additional)
                    }
                }
                4 => {
                    let mut index = 0;
                    let mut content = vec![];
                    let additional = if let Some(count) = additional {
                        // array with definite length: copy the (canonicalized) values
                        for _ in 0..count {
                            index += canonicalize_int(&input[index..], &mut content)?;
                        }
                        count
                    } else {
                        // array with indefinite length: count the number of entries
                        // for the definite-length tag and copy the (canonicalized)
                        // values
                        let mut count = 0;
                        while *input.get(index).ok_or_else(oe_inv_arg(
                            "invalid CBOR: incomplete indefinite-length array",
                        ))? != 0xFF
                        {
                            count += 1;
                            index += canonicalize_int(&input[index..], &mut content)?;
                        }
                        index += 1; // for the 0xFF
                        count
                    };
                    (content, index, additional)
                }
                5 => {
                    // copy the (canonicalized) key/value pairs, ordered by their byte
                    // representations
                    let mut index = 0;
                    let mut content = vec![];
                    let additional = if let Some(count) = additional {
                        // dict with definite length: copy and sort the (canonicalized)
                        // key/value pairs
                        for _ in 0..count {
                            let mut pair = vec![];
                            index += canonicalize_int(&input[index..], &mut pair)?;
                            index += canonicalize_int(&input[index..], &mut pair)?;
                            content.push(pair);
                        }
                        count
                    } else {
                        // dict with indefinite length: count the number of pairs for
                        // the definite-length tag and copy/sort the (canonicalized)
                        // key/value pairs
                        let mut count = 0;
                        while *input.get(index).ok_or_else(oe_inv_arg(
                            "invalid CBOR: incomplete indefinite-length dict",
                        ))? != 0xFF
                        {
                            count += 1;
                            let mut pair = vec![];
                            index += canonicalize_int(&input[index..], &mut pair)?;
                            index += canonicalize_int(&input[index..], &mut pair)?;
                            content.push(pair);
                        }
                        index += 1; // for the 0xFF
                        count
                    };
                    content.sort();
                    (content.into_iter().flatten().collect(), index, additional)
                }
                6 => {
                    // semantic tag
                    let additional = additional.ok_or_else(oe_inv_arg(
                        "invalid CBOR: indefinite length tag for semantic tag",
                    ))?;
                    let mut content = vec![];
                    let len = canonicalize_int(input, &mut content)?;
                    (content, len, additional)
                }
                _ => unreachable!(),
            };

        // Determine with what format we should write the additional data of the
        // tag. For all but major type 7, we write it in the shortest form
        // passible; for type 7, the minor type determines which floating point
        // type is used so we can't remove this information.
        let minor = if major == 7 {
            minor
        } else if additional <= 23 {
            additional as u8
        } else if additional <= 0xFF {
            24
        } else if additional <= 0xFFFF {
            25
        } else if additional <= 0xFFFF_FFFF {
            26
        } else {
            27
        };

        // Construct the canonicalized tag.
        output.push((major << 5) | minor);
        match minor {
            24 => u64_as_bytes(additional, 1, output),
            25 => u64_as_bytes(additional, 2, output),
            26 => u64_as_bytes(additional, 4, output),
            27 => u64_as_bytes(additional, 8, output),
            _ => {}
        }

        // Add the canonicalized content.
        output.extend(content.into_iter());

        //eprintln!("EXIT  canonicalize_int({:?}, {:?})", input, output);
        Ok(tag_len + input_len)
    }

    /// Canonicalizes the given CBOR value.
    pub fn canonizalize(input: &[u8]) -> Result<Vec<u8>> {
        //eprintln!("--------------------------");
        let mut output = vec![];
        let len = canonicalize_int(input, &mut output)?;
        if len != input.len() {
            inv_arg("invalid CBOR: garbage after end")?;
        }
        Ok(output)
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        fn cbor_cmp_golden(a: &[u8], b: &[u8]) {
            let a: serde_cbor::value::Value = serde_cbor::from_slice(a).unwrap();
            let b: serde_cbor::value::Value = serde_cbor::from_slice(b).unwrap();
            assert_eq!(a, b);
        }

        #[test]
        fn test() {
            let data = vec![
                vec![0x00],
                vec![0x01],
                vec![0x0A],
                vec![0x17],
                vec![0x18, 0x18],
                vec![0x18, 0x19],
                vec![0x18, 0x64],
                vec![0x18, 0xFF],
                vec![0x19, 0x01, 0x00],
                vec![0x19, 0x03, 0xE8],
                vec![0x19, 0xFF, 0xFF],
                vec![0x1A, 0x00, 0x01, 0x00, 0x00],
                vec![0x1A, 0x00, 0x0F, 0x42, 0x40],
                vec![0x1A, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x1B, 0x00, 0x00, 0x00, 0x01, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x1B, 0x00, 0x00, 0x00, 0xE8, 0xD4, 0xA5, 0x10, 0x00],
                vec![0x1B, 0x00, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x1B, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x1B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x20],
                vec![0x29],
                vec![0x38, 0x63],
                vec![0x39, 0x03, 0xE7],
                vec![0x3A, 0x7F, 0xFF, 0xFF, 0xFF],
                vec![0x3B, 0x00, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x3B, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x3B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0x40],
                vec![0x44, 0x01, 0x02, 0x03, 0x04],
                vec![0x45, 0x00, 0x01, 0x02, 0x03, 0x04],
                vec![0x45, 0x01, 0x02, 0x03, 0x04, 0x05],
                vec![
                    0x58, 0x19, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
                    0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                    0x18,
                ],
                vec![0x60],
                vec![0x61, 0x61],
                vec![0x62, 0x22, 0x5C],
                vec![0x62, 0xC3, 0xBC],
                vec![0x63, 0xE6, 0xB0, 0xB4],
                vec![0x64, 0x49, 0x45, 0x54, 0x46],
                vec![0x64, 0xF0, 0x90, 0x85, 0x91],
                vec![0x69, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x69, 0x6E, 0x67],
                vec![0x80],
                vec![0x82, 0x01, 0x02],
                vec![0x82, 0x61, 0x61, 0xA1, 0x61, 0x62, 0x61, 0x63],
                vec![0x82, 0x61, 0x61, 0xBF, 0x61, 0x62, 0x61, 0x63, 0xFF],
                vec![0x83, 0x01, 0x02, 0x03],
                vec![0x83, 0x01, 0x82, 0x02, 0x03, 0x82, 0x04, 0x05],
                vec![0x83, 0x01, 0x82, 0x02, 0x03, 0x9F, 0x04, 0x05, 0xFF],
                vec![0x83, 0x01, 0x9F, 0x02, 0x03, 0xFF, 0x82, 0x04, 0x05],
                vec![
                    0x98, 0x19, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                    0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
                    0x18, 0x18, 0x19,
                ],
                vec![
                    0x9F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
                    0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x18,
                    0x18, 0x19, 0xFF,
                ],
                vec![0x9F, 0x01, 0x82, 0x02, 0x03, 0x82, 0x04, 0x05, 0xFF],
                vec![0x9F, 0x01, 0x82, 0x02, 0x03, 0x9F, 0x04, 0x05, 0xFF, 0xFF],
                vec![0x9F, 0xFF],
                vec![0xA0],
                vec![0xA1, 0x01, 0x02],
                vec![0xA1, 0xA1, 0x61, 0x62, 0x01, 0xA1, 0x61, 0x62, 0x01],
                vec![0xA2, 0x01, 0x02, 0x03, 0x04],
                vec![0xA2, 0x61, 0x31, 0x02, 0x61, 0x33, 0x04],
                vec![0xA2, 0x61, 0x61, 0x01, 0x61, 0x62, 0x82, 0x02, 0x03],
                vec![
                    0xA5, 0x61, 0x61, 0x61, 0x41, 0x61, 0x62, 0x61, 0x42, 0x61, 0x63, 0x61, 0x43,
                    0x61, 0x64, 0x61, 0x44, 0x61, 0x65, 0x61, 0x45,
                ],
                vec![
                    0xB8, 0x19, 0x00, 0x61, 0x30, 0x01, 0x61, 0x31, 0x02, 0x61, 0x32, 0x03, 0x61,
                    0x33, 0x04, 0x61, 0x34, 0x05, 0x61, 0x35, 0x06, 0x61, 0x36, 0x07, 0x61, 0x37,
                    0x08, 0x61, 0x38, 0x09, 0x61, 0x39, 0x0A, 0x62, 0x31, 0x30, 0x0B, 0x62, 0x31,
                    0x31, 0x0C, 0x62, 0x31, 0x32, 0x0D, 0x62, 0x31, 0x33, 0x0E, 0x62, 0x31, 0x34,
                    0x0F, 0x62, 0x31, 0x35, 0x10, 0x62, 0x31, 0x36, 0x11, 0x62, 0x31, 0x37, 0x12,
                    0x62, 0x31, 0x38, 0x13, 0x62, 0x31, 0x39, 0x14, 0x62, 0x32, 0x30, 0x15, 0x62,
                    0x32, 0x31, 0x16, 0x62, 0x32, 0x32, 0x17, 0x62, 0x32, 0x33, 0x18, 0x18, 0x62,
                    0x32, 0x34,
                ],
                vec![
                    0xBF, 0x61, 0x61, 0x01, 0x61, 0x62, 0x9F, 0x02, 0x03, 0xFF, 0xFF,
                ],
                vec![
                    0xBF, 0x63, 0x46, 0x75, 0x6E, 0xF5, 0x63, 0x41, 0x6D, 0x74, 0x21, 0xFF,
                ],
                vec![0xBF, 0xFF],
                vec![
                    0xC0, 0x74, 0x32, 0x30, 0x31, 0x33, 0x2D, 0x30, 0x33, 0x2D, 0x32, 0x31, 0x54,
                    0x32, 0x30, 0x3A, 0x30, 0x34, 0x3A, 0x30, 0x30, 0x5A,
                ],
                vec![0xC1, 0x00],
                vec![0xC1, 0x1A, 0x51, 0x4B, 0x67, 0xB0],
                vec![0xC1, 0xFB, 0x41, 0xD4, 0x52, 0xD9, 0xEC, 0x20, 0x00, 0x00],
                vec![0xC2, 0x41, 0x00],
                vec![
                    0xC2, 0x49, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                vec![
                    0xC3, 0x49, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                vec![
                    0xC3, 0x49, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                vec![0xC4, 0x82, 0x20, 0x01],
                vec![0xC4, 0x82, 0x20, 0x18, 0x65],
                vec![0xC4, 0x82, 0x20, 0x19, 0x03, 0xE9],
                vec![0xC4, 0x82, 0x20, 0x20],
                vec![
                    0xC4, 0x82, 0x20, 0xC2, 0x49, 0x09, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xF7,
                ],
                vec![0xC5, 0x82, 0x20, 0x03],
                vec![0xD7, 0x44, 0x01, 0x02, 0x03, 0x04],
                vec![0xD8, 0x18, 0x45, 0x64, 0x49, 0x45, 0x54, 0x46],
                vec![
                    0xD8, 0x20, 0x76, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77,
                    0x2E, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D,
                ],
                vec![
                    0xD8, 0x20, 0x77, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77,
                    0x2E, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x2F,
                ],
                vec![0xD8, 0x23, 0x61, 0x61],
                vec![0xD8, 0x40, 0xD8, 0x40, 0x9F, 0xFF],
                vec![0xD9, 0x01, 0x00, 0x01],
                vec![0xD9, 0xFF, 0xFF, 0x63, 0x66, 0x6F, 0x6F],
                vec![0xF4],
                vec![0xF5],
                vec![0xF6],
                vec![0xF7],
                vec![0xF9, 0x00, 0x00],
                vec![0xF9, 0x00, 0x01],
                vec![0xF9, 0x04, 0x00],
                vec![0xF9, 0x3C, 0x00],
                vec![0xF9, 0x3E, 0x00],
                vec![0xF9, 0x7B, 0xFF],
                vec![0xF9, 0x7C, 0x00],
                vec![0xF9, 0x7E, 0x00],
                vec![0xF9, 0x80, 0x00],
                vec![0xF9, 0xC4, 0x00],
                vec![0xF9, 0xFC, 0x00],
                vec![0xFA, 0x47, 0xC3, 0x50, 0x00],
                vec![0xFA, 0x7F, 0x7F, 0xFF, 0xFF],
                vec![0xFA, 0x7F, 0x80, 0x00, 0x00],
                vec![0xFA, 0x7F, 0xC0, 0x00, 0x00],
                vec![0xFA, 0xFF, 0x80, 0x00, 0x00],
                vec![0xFB, 0x3E, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0x3F, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0x3F, 0xF1, 0x99, 0x99, 0x99, 0x99, 0x99, 0x9A],
                vec![0xFB, 0x3F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0x43, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0x47, 0xEF, 0xFF, 0xFF, 0xE0, 0x00, 0x00, 0x00],
                vec![0xFB, 0x7E, 0x37, 0xE4, 0x3C, 0x88, 0x00, 0x75, 0x9C],
                vec![0xFB, 0x7F, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
                vec![0xFB, 0x7F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0x7F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0xC0, 0x10, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66],
                vec![0xFB, 0xC3, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0xFF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                vec![0xFB, 0xFF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ];
            for cbor in data {
                cbor_cmp_golden(&cbor[..], &canonizalize(&cbor[..]).unwrap());
            }
        }
    }
}

const EMPTY_CBOR: &[u8] = &[0xA0];

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
            self.cbor = cbor_canon::canonizalize(&output)?;
            Ok(())
        }
    }

    /// Sets the JSON/CBOR data field by means of a CBOR string.
    pub fn set_cbor(&mut self, cbor: impl AsRef<[u8]>) -> Result<()> {
        self.cbor = cbor_canon::canonizalize(cbor.as_ref())?;
        Ok(())
    }

    /// Provides a reference to the binary argument vector.
    pub fn set_args(&mut self, args: impl Into<Vec<Vec<u8>>>) {
        self.args = args.into();
    }

    /// Resets the CBOR to an empty object.
    pub fn clear_cbor(&mut self) {
        self.set_cbor(EMPTY_CBOR).unwrap();
    }

    /// Clears the binary arguments vector.
    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    /// Clears the CBOR object and binary arguments vector.
    pub fn clear(&mut self) {
        self.clear_cbor();
        self.clear_args();
    }

    /// Copies the data from another ArbData to this one.
    pub fn copy_from(&mut self, src: &ArbData) {
        self.cbor = src.get_cbor().to_vec();
        self.args = src.get_args().to_vec();
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
            cbor: cbor_canon::canonizalize(&ArbData::scan_json_arg(&mut iterator)?)?,
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
        let data = ArbData::from_json(json!({"test": 42}).to_string(), args.clone()).unwrap();
        let data_ = ArbData::from_json(json!({"test": 42}).to_string(), args).unwrap();
        assert_eq!(data, data_);
    }
}
