use std::cell::UnsafeCell;

struct LocalCounter(UnsafeCell<u64>);

impl LocalCounter {
    fn increment(&self) -> u64 {
        let pointer = self.0.get();
        // SAFETY: UnsafeCell permite la mutación compartida; el tipo no es Sync
        // y el método no llama código reentrante mientras modifica el valor.
        unsafe {
            *pointer += 1;
            *pointer
        }
    }
}

let counter = LocalCounter(UnsafeCell::new(41));
assert_eq!(counter.increment(), 42);
