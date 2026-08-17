#![doc = include_str!("../doctests/book.md")]
#![allow(clippy::needless_doctest_main, clippy::test_attr_in_doctest)]

/// Identifica la edición del corpus de ejemplos que acompaña al libro.
pub const BOOK_CODE_EDITION: &str = "1.0";

#[cfg(test)]
mod tests {
    use super::BOOK_CODE_EDITION;

    #[test]
    fn edition_is_declared() {
        assert_eq!(BOOK_CODE_EDITION, "1.0");
    }
}
