pub mod fuzzing;
#[cfg(test)]
mod property;
#[cfg(test)]
mod rstest;
#[cfg(test)]
mod snapshot;

/// Gets the first name of `name`.
///
/// # Example
///
/// **Great** example! _Fantastic_.
///
/// ```
/// use rust_testing_explored::first_name;
///
/// assert_eq!(Some("John"), first_name("John Doe Jr."));
/// ```
///
/// # Broken examples
///
/// ## Totally ignored
///
/// ```ignore
/// use rust_testing_explored::first_name;
///
/// assert_eq!(Some("Jane"), first_name("John Doe Jr."));
/// ```
///
/// ## Compiles but panics when run
///
/// ```should_panic
/// use rust_testing_explored::first_name;
///
/// assert_eq!(Some("Jane"), first_name("John Doe Jr."));
/// ```
///
/// ## Does not compile
///
/// ```compile_fail
/// use zig_testing_explored::first_name;
///
/// assert_eq!(Some("Jane"), first_name("John Doe Jr."));
/// ```
pub fn first_name(name: &str) -> Option<&str> {
    name.split_once(' ').map(first_element) //.or(Some(""))
}

fn first_element<T>(pair: (T, T)) -> T {
    pair.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_name_loop() {
        let cases = vec![
            ("John Doe", Some("John")),
            ("John Doe Sr.", Some("John")),
            ("", None),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, first_name(input));
        }
    }
}
