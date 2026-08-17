use std::fmt::Display;

trait Encoder {
    fn encode<T: Display>(&self, value: T) -> String;
}

fn use_dynamic(_encoder: &dyn Encoder) {}

fn main() {}
