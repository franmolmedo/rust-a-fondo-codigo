fn min_max(values: &[i32]) -> Option<(i32, i32)> {
    let minimum = *values.iter().min()?;
    let maximum = *values.iter().max()?;
    Some((minimum, maximum))
}
