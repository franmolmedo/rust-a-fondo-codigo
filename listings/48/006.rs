use std::marker::PhantomData;
use std::ptr::NonNull;

struct RawOwner<T> {
    pointer: NonNull<T>,
    owns: PhantomData<T>,
}

// SAFETY: the complete proof must justify unique ownership, access, and Drop.
unsafe impl<T: Send> Send for RawOwner<T> {}
// SAFETY: only shared &T references may be obtained from &RawOwner<T>.
unsafe impl<T: Sync> Sync for RawOwner<T> {}

fn main() {}
