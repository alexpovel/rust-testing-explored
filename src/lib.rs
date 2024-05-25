/// Gets the first name of `name`.
///
/// # Example
///
/// ```
/// use rust_testing_explored::first_name;
///
/// assert_eq!(Some("John"), first_name("John Doe Jr."));
/// ```
pub fn first_name(name: &str) -> Option<&str> {
    name.split_once(' ').map(first_element)
}

fn first_element<T>(pair: (T, T)) -> T {
    pair.0
}

#[cfg(test)]
mod test {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("John Doe", Some("John"))]
    #[case("John Doe Sr.", Some("John"))]
    #[case("", None)]
    fn test_first_name(#[case] input: &str, #[case] expected: Option<&str>) {
        assert_eq!(expected, first_name(input));
    }
}
