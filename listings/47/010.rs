use std::ffi::c_void;

#[repr(C)]
struct ApiV1 {
    abi_version: u32,
    struct_size: usize,
    context: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

let api = ApiV1 {
    abi_version: 1,
    struct_size: std::mem::size_of::<ApiV1>(),
    context: std::ptr::null_mut(),
    destroy: None,
};
assert_eq!(api.abi_version, 1);
