fn require_identity<F>(_callback: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
}

fn main() {
    let fallback = String::from("capturado");
    let ignores_input = |_input: &str| fallback.as_str();
    require_identity(ignores_input);
}
