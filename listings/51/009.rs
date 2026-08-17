macro_rules! classify_2024 {
    ($value:expr) => { "general" };
    (const $value:expr) => { "const" };
    (_) => { "placeholder" };
}

macro_rules! preserve_2021 {
    ($value:expr_2021) => { "general" };
    (const $value:expr) => { "const" };
    (_) => { "placeholder" };
}

assert_eq!(classify_2024!(const { 4 }), "general");
assert_eq!(classify_2024!(_), "general");
assert_eq!(preserve_2021!(const { 4 }), "const");
assert_eq!(preserve_2021!(_), "placeholder");
