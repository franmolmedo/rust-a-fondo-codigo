mod formatting {
    fn normalize_label(input: &str) -> String {
        input.trim().to_lowercase()
    }
}

fn main() {
    formatting::normalize_label("  Rust  ");
    // error[E0603]: function `normalize_label` is private
}
