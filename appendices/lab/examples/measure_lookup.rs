use rust_appendix_lab::performance::{SortedIndex, count_hashed, count_linear};
use std::collections::HashSet;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn sample(operation: impl FnOnce() -> usize, expected: usize) -> Duration {
    let start = Instant::now();
    let result = black_box(operation());
    let elapsed = start.elapsed();
    assert_eq!(result, expected);
    elapsed
}

fn main() {
    // Fixed, non-cryptographic input generation makes the workload repeatable.
    let values: Vec<u32> = (0..4096).map(|index| (index * 7919) % 8192).collect();
    let queries: Vec<u32> = (0..2048)
        .map(|index| (index * 104729 + 17) % 8192)
        .collect();
    let sorted = SortedIndex::new(&values);
    let hashed: HashSet<u32> = values.iter().copied().collect();
    let expected = count_linear(&values, &queries);
    assert_eq!(sorted.count(&queries), expected);
    assert_eq!(count_hashed(&hashed, &queries), expected);

    let names = [
        "linear",
        "sorted-query",
        "hash-query",
        "sorted-total",
        "hash-total",
    ];
    let mut samples: [Vec<Duration>; 5] = std::array::from_fn(|_| Vec::with_capacity(9));
    for round in 0..10 {
        for position in 0..names.len() {
            // Rotate the order; discard the first round as warm-up.
            let strategy = (round + position) % names.len();
            let elapsed = match strategy {
                0 => sample(
                    || count_linear(black_box(&values), black_box(&queries)),
                    expected,
                ),
                1 => sample(|| black_box(&sorted).count(black_box(&queries)), expected),
                2 => sample(
                    || count_hashed(black_box(&hashed), black_box(&queries)),
                    expected,
                ),
                3 => sample(
                    || SortedIndex::new(black_box(&values)).count(black_box(&queries)),
                    expected,
                ),
                4 => sample(
                    || {
                        let index = black_box(&values).iter().copied().collect();
                        count_hashed(&index, black_box(&queries))
                    },
                    expected,
                ),
                _ => unreachable!(),
            };
            if round != 0 {
                samples[strategy].push(elapsed);
            }
        }
    }
    println!(
        "values={} queries={} hits={expected} samples=9",
        values.len(),
        queries.len()
    );
    println!("strategy,median_us,min_us,max_us");
    for (name, mut times) in names.into_iter().zip(samples) {
        times.sort_unstable();
        println!(
            "{name},{:.3},{:.3},{:.3}",
            times[4].as_secs_f64() * 1e6,
            times[0].as_secs_f64() * 1e6,
            times[8].as_secs_f64() * 1e6
        );
    }
}
