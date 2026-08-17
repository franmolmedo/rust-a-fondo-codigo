#![deny(unreachable_patterns)]

fn classify(number: i32) -> &'static str {
    match number {
        _ => "anything",
        0 => "zero",
        // error: unreachable pattern
    }
}

fn main() {
    let _ = classify(0);
}
