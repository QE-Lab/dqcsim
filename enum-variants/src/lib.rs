//! This crate adds a trait for basic enum reflection. Normally used in
//! conjunction with [`enum_variants_derive`](../enum_variants_derive/index.html)
//! for the derive macro.

use failure::Fail;

pub use enum_variants_derive::*;

/// Error structure used for reporting an invalid conversion from a string to
/// an enum variant.
#[derive(Debug, Fail, PartialEq)]
pub enum EnumVariantError {
    #[fail(display = "{}", 0)]
    ParseError(String),
}

/// Trait for adding some reflection features to enums.
///
/// This is normally implemented using `#[derive(EnumVariants)]` from the
/// `enum-variants-derive` crate.
///
/// # Examples
///
/// ```rust
/// use enum_variants::{EnumVariants, EnumVariantError};
/// use std::str::FromStr;
///
/// #[derive(EnumVariants, Clone, PartialEq, Debug)]
/// enum MyEnum {
///     Foo,
///     Bar,
///     Baz
/// }
///
/// // Type name as string:
/// assert_eq!(MyEnum::type_name(), "MyEnum");
///
/// // To and from string:
/// assert_eq!(MyEnum::Foo.to_string(), "Foo");
/// assert_eq!(MyEnum::from_str("Foo").unwrap(), MyEnum::Foo);
///
/// // Fuzzy matching for UX:
/// assert_eq!(MyEnum::from_str("f").unwrap(), MyEnum::Foo); // Abbreviated
/// MyEnum::from_str("ba").unwrap_err();                     // Ambiguous
/// MyEnum::from_str("foobar").unwrap_err();                 // Unknown
/// ```
pub trait EnumVariants
where
    Self: Sized + Clone + 'static,
{
    /// Returns the name of this type as a string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum Test { A, B, C }
    ///
    /// assert_eq!(Test::type_name(), "Test");
    /// ```
    fn type_name() -> &'static str;

    /// Returns a vector containing two-tuples of the variant name as a string
    /// slice and a reference to the variant itself.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum Test { A, B, C }
    ///
    /// assert_eq!(
    ///     Test::variant_map(),
    ///     vec![
    ///         ("A", &Test::A),
    ///         ("B", &Test::B),
    ///         ("C", &Test::C)
    ///     ]
    /// );
    /// ```
    fn variant_map() -> Vec<(&'static str, &'static Self)>;

    /// Returns a vector containing the possible variant names as string
    /// slices. The default implementation is based on `variant_map()`, but
    /// `#[derive(EnumVariants)]` generates a slightly more efficient
    /// implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum Test { A, B, C }
    ///
    /// assert_eq!(Test::variants(), vec!["A", "B", "C"]);
    /// ```
    fn variants() -> Vec<&'static str> {
        Self::variant_map().into_iter().map(|x| x.0).collect()
    }

    /// Returns the name of this type as a lowercase string slice. Intended for
    /// case-insensitive matching.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum Test { A, B, C }
    ///
    /// assert_eq!(Test::type_name_lower(), "test");
    /// ```
    fn type_name_lower() -> &'static str;

    /// Returns a vector containing two-tuples of the variant name as a
    /// lowercase string slice and a reference to the variant itself. Intended
    /// for case-insensitive matching.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum Test { A, B, C }
    ///
    /// assert_eq!(
    ///     Test::variant_map_lower(),
    ///     vec![
    ///         ("a", &Test::A),
    ///         ("b", &Test::B),
    ///         ("c", &Test::C)
    ///     ]
    /// );
    /// ```
    fn variant_map_lower() -> Vec<(&'static str, &'static Self)>;

    /// Returns a vector containing the possible variant names as lowercase
    /// string slices. Intended for case-insensitive matching. The default
    /// implementation is based on `variant_map()`, but
    /// `#[derive(EnumVariants)]` generates a slightly more efficient
    /// implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum Test { A, B, C }
    ///
    /// assert_eq!(Test::variants_lower(), vec!["a", "b", "c"]);
    /// ```
    fn variants_lower() -> Vec<&'static str> {
        Self::variant_map_lower().into_iter().map(|x| x.0).collect()
    }

    /// Returns the enum variant that best matches the given input string.
    ///
    /// This is intended to be used for user interfaces, for instance
    /// converting a command line parameter to the respective internal enum.
    /// Therefore, best-effort matching is performend: the match is case
    /// insensitive, and abbreviations are allowed as long as they are not
    /// ambiguous. Furthermore, the error messages returned are properly
    /// formatted and informative, and can thus be returned directly to the
    /// user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use enum_variants::{EnumVariants, EnumVariantError};
    ///
    /// #[derive(EnumVariants, Clone, PartialEq, Debug)]
    /// enum MyEnum {
    ///     Foo,
    ///     Bar,
    ///     Baz
    /// }
    ///
    /// assert_eq!(MyEnum::variant_from_str_fuzzy("Foo"), Ok(MyEnum::Foo));
    /// assert_eq!(MyEnum::variant_from_str_fuzzy("f"), Ok(MyEnum::Foo));
    /// assert_eq!(
    ///     MyEnum::variant_from_str_fuzzy("ba"),
    ///     Err(EnumVariantError::ParseError(
    ///         "ba is an ambiguous MyEnum. Did you mean bar or baz?"
    ///         .to_string()
    ///     ))
    /// );
    /// assert_eq!(
    ///     MyEnum::variant_from_str_fuzzy("banana"),
    ///     Err(EnumVariantError::ParseError(
    ///         "banana is not a valid MyEnum. Valid values are foo, bar, or baz."
    ///         .to_string()
    ///     ))
    /// );
    /// ```
    fn variant_from_str_fuzzy(s: impl Into<String>) -> Result<Self, EnumVariantError> {
        // Match using a lowercase version of the provided string, so we match
        // case insensitively.
        let mut s: String = s.into();
        s.make_ascii_lowercase();

        // Record possible matches.
        let mut matches: Vec<(&'static str, &'static Self)> = vec![];
        for var in Self::variant_map_lower() {
            if var.0.starts_with(&s) {
                matches.push(var);
            }
        }

        // We're expecting one match; more is ambiguous, less is no match.
        match matches.len() {
            0 => Err(EnumVariantError::ParseError(format!(
                "{} is not a valid {}. Valid values are {}.",
                s,
                Self::type_name(),
                friendly_enumerate(Self::variants_lower().into_iter(), Some("or"))
            ))),
            1 => Ok(unsafe { matches.get_unchecked(0) }.1.clone()),
            _ => Err(EnumVariantError::ParseError(format!(
                "{} is an ambiguous {}. Did you mean {}?",
                s,
                Self::type_name(),
                friendly_enumerate(matches.into_iter().map(|x| x.0), Some("or"))
            ))),
        }
    }
}

/// Turns a string list into its natural language equivalent.
///
/// The optional conjunction is placed between the final comma and final
/// string, if specified. An empty list returns `"nothing"`.
///
/// # Examples
///
/// ```rust
/// use enum_variants::friendly_enumerate;
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
        },
        None => "".to_string()
    };

    // Handle the empty list case, and the first word if the list is not
    // empty.
    let mut s: String;
    match items.next() {
        None => return "<nothing>".to_string(),
        Some(x) => s = x.into()
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
            },
            None => break
        }
        first = false;
    }
    s
}
