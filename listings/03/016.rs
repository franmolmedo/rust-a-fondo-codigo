fn main() {
    let result = 0.1_f64 + 0.2;
    assert!((result - 0.3).abs() < 1e-12);
}
