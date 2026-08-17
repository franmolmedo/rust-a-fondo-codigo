fn numbers(reverse: bool) -> impl Iterator<Item = u32> {
    if reverse {
        (0..3).rev() // Rev<Range<u32>>
    } else {
        0..3         // Range<u32>
    }
}

fn main() {
    let _ = numbers(true);
}
