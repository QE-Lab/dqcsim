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

}
