fn pair<'a, T>(anchor: &'a (), value: T) -> impl Sized + use<'a, T> {
    (anchor, value)
}
