fn repeat<F>(times: usize, mut operation: F)
where
    F: FnMut(usize),
{
    for index in 0..times {
        operation(index);
    }
}

fn main() {
    let mut total = 0;
    repeat(4, |index| total += index);
    assert_eq!(total, 6);
}
