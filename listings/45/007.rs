use std::cell::UnsafeCell;

struct LocalCounter(UnsafeCell<u64>);

impl LocalCounter {
    fn increment(&self) -> Option<u64> {
        let pointer = self.0.get();
        // SAFETY: UnsafeCell permits shared mutation; the type is not Sync,
        // and the method calls no reentrant code while changing the value.
        unsafe {
            let next = (*pointer).checked_add(1)?;
            *pointer = next;
            Some(next)
        }
    }
}

let counter = LocalCounter(UnsafeCell::new(41));
assert_eq!(counter.increment(), Some(42));
