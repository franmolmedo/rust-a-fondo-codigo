use std::mem::MaybeUninit;

let mut slot = MaybeUninit::<String>::uninit();
let initialized: &mut String = slot.write(String::from("ready"));
initialized.push('!');

// SAFETY: `write` produjo un String válido y no se ha movido ni destruido.
let value = unsafe { slot.assume_init() };
assert_eq!(value, "ready!");
