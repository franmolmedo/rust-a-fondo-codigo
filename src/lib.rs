#![doc = include_str!("../doctests/book.md")]
#![allow(clippy::needless_doctest_main, clippy::test_attr_in_doctest)]

/// Identifica la edición del corpus de ejemplos.
pub const COURSE_CODE_EDITION: &str = "2.0";

#[cfg(test)]
mod tests {
    use super::COURSE_CODE_EDITION;

    #[test]
    fn edition_is_declared() {
        assert_eq!(COURSE_CODE_EDITION, "2.0");
    }
}
