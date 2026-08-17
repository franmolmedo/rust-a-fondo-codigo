fn copied<'a>(values: &'a [u32]) -> impl Iterator<Item = u32> + use<'a> {
    values.iter().copied()
}

assert_eq!(copied(&[2, 3, 5]).sum::<u32>(), 10);
