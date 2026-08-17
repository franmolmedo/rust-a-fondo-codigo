fn main() {
    let optional = Some("Rust").map(str::len);
    let fallible: Result<usize, &str> = Ok("Rust").map(str::len);
    let many: Vec<usize> = ["Rust", "book"]
        .into_iter()
        .map(str::len)
        .collect();

    assert_eq!(optional, Some(4));
    assert_eq!(fallible, Ok(4));
    assert_eq!(many, [4, 4]);
}
