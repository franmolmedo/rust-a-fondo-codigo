fn replace_after_successful_build<T, F>(slot: &mut T, build: F)
where
    F: FnOnce(&T) -> T,
{
    let replacement = build(slot); // si hace panic, `slot` sigue intacto
    *slot = replacement;
}
