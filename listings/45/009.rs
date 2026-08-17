let values = [10_u32, 20, 30];
let begin = values.as_ptr();

// SAFETY: dos elementos están dentro de la misma array viva.
let third = unsafe { begin.add(2) };
// SAFETY: `third` señala un `u32` inicializado y compartidamente legible.
assert_eq!(unsafe { third.read() }, 30);

// SAFETY: one-past puede calcularse, pero no leerse.
let end = unsafe { begin.add(values.len()) };
assert_eq!(end, unsafe { third.add(1) });
