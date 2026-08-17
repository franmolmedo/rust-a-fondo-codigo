trait Category {
    fn category(&self) -> &'static str;
}

impl<T> Category for T {
    fn category(&self) -> &'static str { "generic" }
}

impl Category for String {
    fn category(&self) -> &'static str { "text" }
}

fn main() {}
