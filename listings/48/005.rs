use std::marker::PhantomData;

struct Writer<'a, T>(PhantomData<&'a mut T>);

fn shorten_element<'a, 'short: 'a>(
    writer: Writer<'a, &'static str>,
) -> Writer<'a, &'short str> {
    writer
}

fn main() {}
