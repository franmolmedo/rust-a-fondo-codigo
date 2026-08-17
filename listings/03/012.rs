fn main() {
    let a: u32 = 10;
    let b: u64 = 20;
    let c = u64::from(a) + b;
    assert_eq!(c, 30);
}
