#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_out_of_range() {
        assert!(Percentage::new(101).is_none());
        assert!(Percentage::new(100).is_some());
    }
}
