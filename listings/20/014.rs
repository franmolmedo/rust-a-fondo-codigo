use std::marker::PhantomData;
use std::mem::size_of;

struct Owns<T> {
    id: usize,
    marker: PhantomData<T>,
}

struct Consumes<T> {
    id: usize,
    marker: PhantomData<fn(T)>,
}

fn main() {
    let owns = Owns::<String> { id: 1, marker: PhantomData };
    let consumes = Consumes::<String> { id: 2, marker: PhantomData };
    assert_eq!((owns.id, consumes.id), (1, 2));
    assert_eq!(size_of::<Owns<String>>(), size_of::<usize>());
    assert_eq!(size_of::<Consumes<String>>(), size_of::<usize>());
}
