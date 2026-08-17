#![allow(invalid_reference_casting)]

let value = 1_i32;
let shared = &value;
let raw = shared as *const i32 as *mut i32;

// UB: la escritura contradice la referencia compartida activa.
unsafe { raw.write(2) };
println!("{shared}");
