use std::ffi::{c_int, c_void};

#[repr(C)]
struct Event {
    kind: c_int,
}

type Callback = unsafe extern "C" fn(
    user_data: *mut c_void,
    event: *const Event,
) -> c_int;

let _type_check: Option<Callback> = None;
