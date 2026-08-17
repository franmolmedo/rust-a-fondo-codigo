use std::cell::Cell;

macro_rules! sum_once {
    ($( $value:expr ),+ $(,)?) => {{
        let mut total = 0;
        $(
            let evaluated_once = $value;
            total += evaluated_once;
        )+
        total
    }};
}

let calls = Cell::new(0);
let next = || {
    calls.set(calls.get() + 1);
    10
};
assert_eq!(sum_once!(next(), next()), 20);
assert_eq!(calls.get(), 2);
