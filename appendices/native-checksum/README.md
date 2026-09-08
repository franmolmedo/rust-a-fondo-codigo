# Native checksum lab

`cargo test -p native-checksum --locked` compiles `native/checksum.c` with
the `cc` build dependency, links it into the test executable and calls it.
It needs a native C compiler: MSVC with the Windows SDK, or a Linux C toolchain.
The checksum is a byte sum modulo 2^32, not an authentication mechanism.

The wrapper borrows a slice for the synchronous call; C does not retain,
mutate or free its storage. This lab does not cover callbacks, owned foreign
objects, C++ exceptions or a stable application-specific struct ABI. Miri
is run on the separate Rust-only audit module, not on this foreign function.

Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
