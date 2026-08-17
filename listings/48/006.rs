use std::marker::PhantomData;
use std::ptr::NonNull;

struct RawOwner<T> {
    pointer: NonNull<T>,
    owns: PhantomData<T>,
}

// SAFETY: la prueba completa debe justificar ownership único, accesos y Drop.
unsafe impl<T: Send> Send for RawOwner<T> {}
// SAFETY: desde &RawOwner<T> solo debe poder obtenerse &T.
unsafe impl<T: Sync> Sync for RawOwner<T> {}

fn main() {}
