#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_surrounding_space() {
        assert_eq!(Name::parse("  Ada  ").unwrap().as_str(), "Ada");
    }
}
