use course_solutions::unsafe_low_level::c48::{RawOwner, try_extend_prefix, try_init_array};
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};

#[test]
fn c48_empty_zst_and_high_alignment_initialization() {
    let empty = try_init_array::<(), (), _, 0>(|_| panic!("empty array must not initialize"));
    assert_eq!(empty, Ok([]));
    let mut calls = 0;
    assert_eq!(
        try_init_array::<_, (), _, 3>(|_| {
            calls += 1;
            Ok(())
        }),
        Ok([(); 3])
    );
    assert_eq!(calls, 3);
    #[repr(align(128))]
    struct Aligned(usize);
    let values = try_init_array::<_, (), _, 3>(|index| Ok(Aligned(index))).unwrap();
    assert_eq!(values[2].0, 2);
    assert_eq!(values.as_ptr().addr() % 128, 0);
}

#[test]
fn c48_element_drop_panic_still_cleans_the_initialized_tail() {
    struct DropSpy<'a> {
        index: usize,
        drops: &'a Cell<usize>,
    }
    impl Drop for DropSpy<'_> {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
            if self.index == 0 {
                panic!("first destructor failed");
            }
        }
    }
    let drops = Cell::new(0);
    // The only shared effect is a valid count, which remains usable on panic.
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = try_init_array::<_, (), _, 4>(|index| {
            if index == 3 {
                Err(())
            } else {
                Ok(DropSpy {
                    index,
                    drops: &drops,
                })
            }
        });
    }));
    assert!(result.is_err());
    assert_eq!(drops.get(), 3);
}

#[test]
fn c48_capacity_overflow_happens_before_initialization() {
    let mut values = vec![()];
    let calls = Cell::new(0);
    // reserve must leave the vector valid on capacity-overflow panic.
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = try_extend_prefix::<_, (), _>(&mut values, usize::MAX, |_| {
            calls.set(calls.get() + 1);
            Ok(())
        });
    }));
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
    assert_eq!(values, [()]);
    fn require_send<T: Send>() {}
    require_send::<RawOwner<Cell<u32>>>();
}
