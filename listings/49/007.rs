macro_rules! doubled {
    ($value:expr) => {{
        let temporary = $value;
        temporary * 2
    }};
}

let temporary = 99;
assert_eq!(doubled!(21), 42);
assert_eq!(temporary, 99);
