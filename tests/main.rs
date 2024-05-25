#[cfg(test)]
mod tests {
    use rust_meetup::first_name;

    #[test]
    fn test_public_first_name_functionality() {
        assert_eq!(Some("John"), first_name("John Doe"))
    }
}
