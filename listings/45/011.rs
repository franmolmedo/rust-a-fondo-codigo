use std::ops::Range;
use std::ptr;

fn copy_within_raw<T: Copy>(
    values: &mut [T],
    source: Range<usize>,
    destination: usize,
) -> bool {
    if source.start > source.end || source.end > values.len() {
        return false;
    }
    let count = source.end - source.start;
    let Some(destination_end) = destination.checked_add(count) else {
        return false;
    };
    if destination_end > values.len() {
        return false;
    }

    let base = values.as_mut_ptr();
    // SAFETY: ambos rangos están dentro de la slice; `copy` admite overlap y
    // `T: Copy` permite usar tanto las copias de origen como las de destino.
    unsafe { ptr::copy(base.add(source.start), base.add(destination), count) };
    true
}

let mut values = [1, 2, 3, 4, 5];
assert!(copy_within_raw(&mut values, 0..4, 1));
assert_eq!(values, [1, 1, 2, 3, 4]);
