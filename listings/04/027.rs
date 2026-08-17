fn main() {
    let label = String::from("retry");
    let attempts = 3_u32;
    let describe = move || format!("{label}: {attempts}");

    assert_eq!(attempts, 3);
    assert_eq!(describe(), "retry: 3");
}
