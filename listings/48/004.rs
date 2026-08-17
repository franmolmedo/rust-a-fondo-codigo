use std::marker::PhantomData;

struct Reader<'a, T>(PhantomData<&'a T>);

fn shorten<'long: 'short, 'short, T>(reader: Reader<'long, T>) -> Reader<'short, T> {
    reader
}

let long = Reader::<'static, String>(PhantomData);
let _short = shorten(long);
