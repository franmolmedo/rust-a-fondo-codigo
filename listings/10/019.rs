#[derive(Debug, PartialEq)]
enum ImportError {
    Empty,
}

type ImportResult<T> = Result<T, ImportError>;

fn import_line(input: &str) -> ImportResult<&str> {
    (!input.trim().is_empty())
        .then_some(input.trim())
        .ok_or(ImportError::Empty)
}

fn main() {
    assert_eq!(import_line(" Rust "), Ok("Rust"));
}
