fn replace_after_successful_build<T, F>(slot: &mut T, build: F)
where
    F: FnOnce(&T) -> T,
{
    let replacement = build(slot); // No replacement is assigned if this call panics.
    *slot = replacement;
}
