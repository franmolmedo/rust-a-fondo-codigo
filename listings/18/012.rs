fn countdown() -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator {
    0_u8..4
}

fn main() {
    let values = countdown();
    assert_eq!(values.len(), 4);
    assert_eq!(countdown().rev().collect::<Vec<_>>(), [3, 2, 1, 0]);
}
