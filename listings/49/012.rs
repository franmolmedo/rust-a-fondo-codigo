macro_rules! expression_kind {
    (const $value:block) => { "const block" };
    (_) => { "placeholder" };
    ($value:expr_2021) => { "legacy expression" };
}

assert_eq!(expression_kind!(1 + 2), "legacy expression");
assert_eq!(expression_kind!(const { 1 + 2 }), "const block");
assert_eq!(expression_kind!(_), "placeholder");
