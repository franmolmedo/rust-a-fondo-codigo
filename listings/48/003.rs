use std::mem::MaybeUninit;

let mut slot = MaybeUninit::<String>::uninit();
let initialized: &mut String = slot.write(String::from("ready"));
initialized.push('!');

// SAFETY: `write` produced a valid String that has not been moved or dropped.
let value = unsafe { slot.assume_init() };
assert_eq!(value, "ready!");
