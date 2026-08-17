fn old_pair<'a, T: 'a>(anchor: &'a (), value: T) -> impl Sized + 'a {
    (anchor, value)
}
