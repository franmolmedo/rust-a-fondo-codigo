macro_rules! count_tokens {
    () => { 0usize };
    ($_head:tt $($tail:tt)*) => { 1usize + count_tokens!($($tail)*) };
}

assert_eq!(count_tokens!(alpha + beta), 3);
