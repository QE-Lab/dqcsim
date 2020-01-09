//! Utility functions that don't belong elsewhere.

use crate::common::error::{inv_arg, Result};

/// Returns the number of bits of a type
const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

/// Returns the log2 for a usize as an Option<usize>.
pub fn log_2(x: usize) -> Option<usize> {
    if x == 0 {
        None
    } else {
        let result = num_bits::<usize>() as u32 - x.leading_zeros() - 1;
        if 2_usize.pow(result) == x {
            Some(result as usize)
        } else {
            None
        }
    }
}
/// Returns a `Complex64` number.
///
/// Shorthand macro which calls the `Complex64` constructor and returns the
/// `Complex64`.
///
/// # Examples
///
/// ```rust
/// use num_complex::Complex64;
///
/// assert_eq!(c!(1.), Complex64::new(1., 0.));
/// assert_eq!(c!(1., 2.), Complex64::new(1., 2.));
/// assert_eq!(c!(0., 1.), Complex64::new(0., 1.));
/// ```
macro_rules! c {
    (($re:expr, $im:expr)) => {
        $crate::core::Complex64::new($re, $im);
    };
    ($re:expr, $im:expr) => {
        $crate::core::Complex64::new($re, $im);
    };
    ($re:expr) => {
        $crate::core::Complex64::new($re, 0.)
    };
}

/// Returns a `Matrix` with the given elements.
///
/// Shorthand for `Matrix::new(vec![...])`. The primary use for this is to
/// prevent `cargo fmt` from making the matrices unreadable.
#[cfg(test)]
macro_rules! matrix {
    ($($($x:tt),+);+) => {
        $crate::core::common::types::Matrix::new(vec![$($(c!($x)),+),+])
    };
    ($($($x:tt),+);+;) => {
        $crate::core::common::types::Matrix::new(vec![$($(c!($x)),+),+])
    };
}

/// Splits a CamelCase name into space-separated lowercase words.
///
/// Abbreviations remain uppercase, as shown in the examples below.
///
/// # Examples
///
/// ```rust
/// use dqcsim::common::util::friendly_name;
///
/// assert_eq!(friendly_name("String"), "string");
/// assert_eq!(friendly_name("EnumVariants"), "enum variants");
/// assert_eq!(friendly_name("TestABC"), "test ABC");
/// assert_eq!(friendly_name("TestABCtestTest"), "test ABCtest test");
/// assert_eq!(friendly_name("TestABC_TestTest"), "test ABC test test");
/// ```
pub fn friendly_name(name: impl AsRef<str>) -> String {
    let name: &str = name.as_ref();
    let mut output = "".to_string();
    let mut prev_upper = false;
    let mut abbreviation = false;
    for c in name.chars() {
        let upper = c.is_ascii_uppercase();
        if upper && prev_upper {
            abbreviation = true;
            let prev = output.pop().unwrap().to_ascii_uppercase();
            output.push(prev);
        }
        if abbreviation && !upper {
            abbreviation = false;
        }
        if abbreviation {
            output.push(c.to_ascii_uppercase());
        } else if upper {
            if let Some(prev) = output.chars().last() {
                if prev != ' ' {
                    output.push(' ');
                }
            }
            output.push(c.to_ascii_lowercase());
        } else if c == '_' {
            output.push(' ');
        } else {
            output.push(c);
        }
        prev_upper = upper;
    }
    output
}

/// Turns a string list into its natural language equivalent.
///
/// The optional conjunction is placed between the final comma and final
/// string, if specified. An empty list returns `"nothing"`.
///
/// # Examples
///
/// ```rust
/// use dqcsim::common::util::friendly_enumerate;
///
/// assert_eq!(
///     friendly_enumerate(vec!["a", "b", "c"].into_iter(), Some("or")),
///     "a, b, or c"
/// );
/// assert_eq!(
///     friendly_enumerate(vec!["x", "y"].into_iter(), Some("and")),
///     "x and y"
/// );
/// assert_eq!(
///     friendly_enumerate(vec!["x", "y"].into_iter(), None),
///     "x, y"
/// );
/// assert_eq!(
///     friendly_enumerate(vec!["foo", "bar", "baz"].into_iter(), None),
///     "foo, bar, baz"
/// );
/// assert_eq!(
///     friendly_enumerate(vec!["one"].into_iter(), Some("and")),
///     "one"
/// );
/// assert_eq!(
///     friendly_enumerate(vec!["one"].into_iter(), None),
///     "one"
/// );
/// let empty: Vec<String> = vec![];
/// assert_eq!(
///     friendly_enumerate(empty.into_iter(), None),
///     "<nothing>"
/// );
/// ```
pub fn friendly_enumerate(
    mut items: impl Iterator<Item = impl Into<String>>,
    conjunction: Option<&str>,
) -> String {
    // Convert the optional conjunction to a string.
    let conjunction: String = match conjunction {
        Some(c) => {
            let mut c: String = c.into();
            c += " ";
            c
        }
        None => "".to_string(),
    };

    // Handle the empty list case, and the first word if the list is not
    // empty.
    let mut s: String;
    match items.next() {
        None => return "<nothing>".to_string(),
        Some(x) => s = x.into(),
    }

    // Handle the rest.
    let mut first = true;
    let mut next = items.next();
    loop {
        let cur = next;
        next = items.next();

        match cur {
            Some(c) => {
                if next.is_none() {
                    if first && !conjunction.is_empty() {
                        // For the case "a <conj> b"
                        s += " ";
                    } else {
                        // For the cases "a, b" and "a, b, <conj> c"
                        s += ", ";
                    }
                    s += &conjunction;
                } else {
                    // For the case "a, b ..."
                    s += ", ";
                }
                let c: String = c.into();
                s += &c;
            }
            None => break,
        }
        first = false;
    }
    s
}

/// Parses a string representing an enum variant into the variant.
///
/// This uses fuzzy matching: case doesn't matter, and it is sufficient to
/// only list the first few characters (just enough to not be ambiguous). The
/// error messages generated are user-friendly as well.
pub fn friendly_enum_parse<E, I>(s: &str) -> Result<E>
where
    E: std::str::FromStr
        + strum::IntoEnumIterator<Iterator = I>
        + named_type::NamedType
        + std::fmt::Display,
    I: Iterator<Item = E>,
{
    // Match using a lowercase version of the provided string, so we match
    // case insensitively.
    let mut s: String = s.into();
    s.make_ascii_lowercase();

    // Record possible matches.
    let mut matches = vec![];
    for var in E::iter() {
        let mut var_str: String = var.to_string();
        var_str.make_ascii_lowercase();
        if var_str.starts_with(&s) {
            matches.push((var_str, var));
        }
    }

    // We're expecting one match; more is ambiguous, less is no match.
    match matches.len() {
        0 => inv_arg(format!(
            "{} is not a valid {}, valid values are {}",
            s,
            friendly_name(E::short_type_name()),
            friendly_enumerate(
                E::iter().map(|e| format!("{}", e).to_lowercase()),
                Some("or")
            )
        )),
        1 => Ok(matches.into_iter().next().unwrap().1),
        _ => inv_arg(format!(
            "{} is an ambiguous {}, it could mean either {}",
            s,
            friendly_name(E::short_type_name()),
            friendly_enumerate(matches.into_iter().map(|x| x.0), Some("or"))
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use named_type::NamedType;
    use named_type_derive::*;
    use std::str::FromStr;
    use strum::ParseError;
    use strum_macros::{Display, EnumIter, EnumString};

    #[derive(NamedType, Display, EnumIter, EnumString, Clone, PartialEq, Debug)]
    enum TestEnum {
        #[strum(to_string = "Foo", serialize = "foo", serialize = "f")]
        Foo,
        #[strum(to_string = "Bar", serialize = "bar", serialize = "b")]
        Bar,
        #[strum(to_string = "Baz", serialize = "baz", serialize = "z")]
        Baz,
    }

    #[test]
    fn num_bits_check() {
        assert_eq!(num_bits::<u8>(), 8);
        assert_eq!(num_bits::<u16>(), 16);
        assert_eq!(num_bits::<u32>(), 32);
        assert_eq!(num_bits::<u64>(), 64);
        assert_eq!(num_bits::<u128>(), 128);
        assert_eq!(num_bits::<TestEnum>(), 8);
        assert_eq!(num_bits::<Option<TestEnum>>(), 8);
        assert_eq!(num_bits::<Option<Option<TestEnum>>>(), 8);
    }

    #[test]
    fn log2_check() {
        assert_eq!(log_2(1), Some(0));
        assert_eq!(log_2(2), Some(1));
        assert_eq!(log_2(3), None);
        assert_eq!(log_2(4), Some(2));
        assert_eq!(log_2(5), None);
        assert_eq!(log_2(6), None);
        assert_eq!(log_2(7), None);
        assert_eq!(log_2(8), Some(3));
    }

    #[test]
    fn parse_error() {
        assert_eq!(TestEnum::from_str("Foo"), Ok(TestEnum::Foo));
        assert_eq!(TestEnum::from_str("f"), Ok(TestEnum::Foo));
        assert_eq!(TestEnum::from_str("ba"), Err(ParseError::VariantNotFound));
        assert_eq!(
            TestEnum::from_str("banana"),
            Err(ParseError::VariantNotFound)
        );
    }

    #[test]
    fn friendlyname() {
        assert_eq!(friendly_name("String"), "string");
        assert_eq!(friendly_name("EnumVariants"), "enum variants");
        assert_eq!(friendly_name("TestABC"), "test ABC");
        assert_eq!(friendly_name("TestABCtestTest"), "test ABCtest test");
        assert_eq!(friendly_name("TestABC_TestTest"), "test ABC test test");
    }

    #[test]
    fn friendlyenumerate() {
        assert_eq!(
            friendly_enumerate(vec!["a", "b", "c"].into_iter(), Some("or")),
            "a, b, or c"
        );
        assert_eq!(
            friendly_enumerate(vec!["x", "y"].into_iter(), Some("and")),
            "x and y"
        );
        assert_eq!(friendly_enumerate(vec!["x", "y"].into_iter(), None), "x, y");
        assert_eq!(
            friendly_enumerate(vec!["foo", "bar", "baz"].into_iter(), None),
            "foo, bar, baz"
        );
        assert_eq!(
            friendly_enumerate(vec!["one"].into_iter(), Some("and")),
            "one"
        );
        assert_eq!(friendly_enumerate(vec!["one"].into_iter(), None), "one");
        let empty: Vec<String> = vec![];
        assert_eq!(friendly_enumerate(empty.into_iter(), None), "<nothing>");
    }

    #[test]
    fn friendlyenumparse() {
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("bar").unwrap(),
            TestEnum::Bar
        );
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("BAR").unwrap(),
            TestEnum::Bar
        );
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("Bar").unwrap(),
            TestEnum::Bar
        );
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("baz").unwrap(),
            TestEnum::Baz
        );
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("BA")
                .unwrap_err()
                .to_string(),
            "Invalid argument: ba is an ambiguous test enum, it could mean either bar or baz"
        );
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("bla")
                .unwrap_err()
                .to_string(),
            "Invalid argument: bla is not a valid test enum, valid values are foo, bar, or baz"
        );
        assert_eq!(
            friendly_enum_parse::<TestEnum, _>("")
                .unwrap_err()
                .to_string(),
            "Invalid argument:  is an ambiguous test enum, it could mean either foo, bar, or baz"
        );
    }
}
