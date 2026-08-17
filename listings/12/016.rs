#[derive(Debug, PartialEq)]
struct Stats {
    count: usize,
    sum: i64,
}

fn stats(values: &[i64]) -> Stats {
    values.iter().copied().fold(
        Stats { count: 0, sum: 0 },
        |stats, value| Stats {
            count: stats.count + 1,
            sum: stats.sum + value,
        },
    )
}

fn main() {
    assert_eq!(stats(&[2, 3, 5]), Stats { count: 3, sum: 10 });
    assert_eq!([2, 3, 5].into_iter().sum::<i32>(), 10);
    assert_eq!([2, 3, 5].into_iter().reduce(i32::max), Some(5));
    assert_eq!([].into_iter().reduce(i32::max), None);
}
