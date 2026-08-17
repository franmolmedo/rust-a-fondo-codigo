fn attach<'a, T>(key: &'a str, value: T) -> impl Sized + use<'a, T> {
    (key, value)
}
