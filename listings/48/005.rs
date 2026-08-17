use std::marker::PhantomData;

struct Writer<'a, T>(PhantomData<&'a mut T>);

fn invent_static<'short>(writer: Writer<'_, &'short str>) -> Writer<'_, &'static str> {
    writer
}

fn main() {}
