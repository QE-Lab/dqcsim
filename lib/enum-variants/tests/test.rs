//  copy of some doctests until rustdoc allows to persist test bins on stable

use enum_variants::*;

#[derive(EnumVariants, Clone, PartialEq, Debug)]
enum TestEnum {
    Foo,
    Bar,
    Baz,
}

#[test]
fn parse_error() {
    assert_eq!(TestEnum::variant_from_str_fuzzy("Foo"), Ok(TestEnum::Foo));
    assert_eq!(TestEnum::variant_from_str_fuzzy("f"), Ok(TestEnum::Foo));
    assert_eq!(
        TestEnum::variant_from_str_fuzzy("ba"),
        Err(EnumVariantError::ParseError(
            "ba is an ambiguous test enum, it could mean either bar or baz".to_string()
        ))
    );
    assert_eq!(
        TestEnum::variant_from_str_fuzzy("banana"),
        Err(EnumVariantError::ParseError(
            "banana is not a valid test enum, valid values are foo, bar, or baz".to_string()
        ))
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
