use std::collections::HashMap;

fn main() {
    let (even, odd): (Vec<u32>, Vec<u32>) =
        (1..=5).partition(|number| number % 2 == 0);
    assert_eq!(even, [2, 4]);
    assert_eq!(odd, [1, 3, 5]);

    let (ids, names): (Vec<u64>, Vec<&str>) =
        [(7, "Ada"), (8, "Grace")].into_iter().unzip();
    assert_eq!(ids, [7, 8]);
    assert_eq!(names, ["Ada", "Grace"]);

    let index: HashMap<u64, &str> = [(7, "Ada"), (8, "Grace")]
        .into_iter()
        .collect();
    assert_eq!(index.get(&8), Some(&"Grace"));
}
