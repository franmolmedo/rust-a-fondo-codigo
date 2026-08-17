use std::path::Path;

fn file_name(path: impl AsRef<Path>) -> Option<String> {
    file_name_core(path.as_ref())
}

fn file_name_core(path: &Path) -> Option<String> {
    path.file_name()?.to_str().map(str::to_owned)
}

fn main() {
    assert_eq!(file_name("reports/book.md"), Some(String::from("book.md")));
    assert_eq!(file_name(Path::new("notes.txt")), Some(String::from("notes.txt")));
}
