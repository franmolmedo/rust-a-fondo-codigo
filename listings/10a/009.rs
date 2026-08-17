use std::error::Error;

type AnyError = Box<dyn Error + Send + Sync + 'static>;

fn assert_transportable<T: Send + Sync + 'static>() {}

fn main() {
    assert_transportable::<AnyError>();
}
