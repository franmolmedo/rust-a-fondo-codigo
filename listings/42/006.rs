use std::fmt::Debug;

fn transform<'a, T>(_: &'a str, value: T) -> impl Debug + use<T>
where
    T: Debug,
{
    value
}
