#[cfg(test)]
mod tests {
    use rust_testing_explored::first_name;

    #[test]
    fn test_public_first_name_functionality() {
        assert_eq!(Some("John"), first_name("John Doe"))
    }
}
