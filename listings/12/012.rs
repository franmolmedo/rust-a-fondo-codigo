fn main() {
    let numbered: Vec<_> = ["Ada", "Grace"]
        .into_iter()
        .enumerate()
        .collect();
    assert_eq!(numbered, [(0, "Ada"), (1, "Grace")]);

    let paired: Vec<_> = ["Ada", "Grace"]
        .into_iter()
        .zip([36, 85])
        .collect();
    assert_eq!(paired, [("Ada", 36), ("Grace", 85)]);

    let window: Vec<_> = (0..10).skip(2).take(3).collect();
    assert_eq!(window, [2, 3, 4]);

    let chained: Vec<_> = [1, 2].into_iter().chain([3, 4]).collect();
    assert_eq!(chained, [1, 2, 3, 4]);
}
