use std::ffi::c_int;
use std::panic::{UnwindSafe, catch_unwind};

const PANIC: c_int = -128;

fn ffi_firewall<F>(operation: F) -> c_int
where
    F: FnOnce() -> c_int + UnwindSafe,
{
    match catch_unwind(operation) {
        Ok(code) => code,
        Err(payload) => {
            // Evita que un payload hostil vuelva a hacer panic en Drop.
            std::mem::forget(payload);
            PANIC
        }
    }
}

assert_eq!(ffi_firewall(|| 7), 7);
assert_eq!(ffi_firewall(|| panic!("boom")), PANIC);
