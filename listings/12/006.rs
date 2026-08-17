use std::cell::Cell;

fn main() {
    let calls = Cell::new(0);
    let pipeline = [1, 2, 3].into_iter().map(|value| {
        calls.set(calls.get() + 1);
        value * 10
    });

    assert_eq!(calls.get(), 0);
    let result: Vec<_> = pipeline.collect();
    assert_eq!(calls.get(), 3);
    assert_eq!(result, [10, 20, 30]);
}
