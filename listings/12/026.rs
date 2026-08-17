fn even_numbers_up_to(limit: u32) -> impl Iterator<Item = u32> {
    (0..=limit).filter(|value| value % 2 == 0)
}

fn main() {
    assert_eq!(even_numbers_up_to(7).collect::<Vec<_>>(), [0, 2, 4, 6]);
}
