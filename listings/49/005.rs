macro_rules! measured {
    ($body:expr) => {{
        let start = std::time::Instant::now();
        let output = $body;
        (output, start.elapsed())
    }};
}

let (answer, _elapsed) = measured!(40 + 2);
assert_eq!(answer, 42);
