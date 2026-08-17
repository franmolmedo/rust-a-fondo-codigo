fn main() {
    let composed = "é";
    let decomposed = "e\u{301}";

    assert_eq!(composed.len(), 2);
    assert_eq!(composed.chars().count(), 1);
    assert_eq!(decomposed.len(), 3);
    assert_eq!(decomposed.chars().count(), 2);
}
