fn main() {
    let x: u8 = 255;

    assert_eq!(x.wrapping_add(1), 0);
    assert_eq!(x.checked_add(1), None);
    assert_eq!(x.saturating_add(1), 255);
    assert_eq!(x.overflowing_add(1), (0, true));
}
