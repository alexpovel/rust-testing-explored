use crate::first_name;
use rstest::*;

#[rstest]
#[case(String::from("John Doe"), Some("John"))] // Use owned `String` to avoid rstest's `magic_conversion`
#[case(String::from("John Doe Sr."), Some("John"))]
#[case(String::from(""), None)]
fn test_first_name_parametrized(#[case] input: String, #[case] expected: Option<&str>) {
    assert_eq!(expected, first_name(input.as_str()));
}
