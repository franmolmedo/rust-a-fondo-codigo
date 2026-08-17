use std::collections::HashMap;

fn main() {
    let vector: Vec<_> = (1..=3).map(|value| value * 2).collect();
    let text: String = ['R', 'u', 's', 't'].into_iter().collect();
    let index: HashMap<_, _> = [(7_u64, "Ada"), (8, "Grace")]
        .into_iter()
        .collect();

    assert_eq!(vector, [2, 4, 6]);
    assert_eq!(text, "Rust");
    assert_eq!(index.get(&7), Some(&"Ada"));
}
