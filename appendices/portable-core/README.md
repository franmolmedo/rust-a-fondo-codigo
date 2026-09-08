# Portable no_std library

`Ring<T: Copy, N>` is a fixed-capacity FIFO without allocation or unsafe code.
Zero capacity is supported and always rejects insertion. Full queues return
the rejected value. `ByteSink` separates byte delivery from queue management;
`flush_to` reports progress and retains unaccepted bytes after adapter failure.
An adapter must not return an error after accepting a byte: otherwise a retry
could duplicate externally visible data.

```console
cargo test -p portable-core --locked
cargo test -p portable-core --features alloc --locked
rustup target add thumbv7em-none-eabihf --toolchain 1.95.0
cargo build -p portable-core --lib --target thumbv7em-none-eabihf --locked
cargo build -p portable-core --lib --features alloc --target thumbv7em-none-eabihf --locked
```

Unit tests may use `std` on the host. Cross-building the library verifies that
the library itself does not require it. The optional `alloc` feature enables
fallible draining into a Vec. Final firmware using it must supply a suitable
allocator and allocation failure policy; the library does not install one.

This is an rlib, not bootable firmware. No board, linker memory map, interrupt
table, panic handler or physical device adapter is provided. Cross-compilation
does not establish correct execution on a board, real-time deadlines or
interrupt-safe access. The queue requires exclusive mutable access for changes.

Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
