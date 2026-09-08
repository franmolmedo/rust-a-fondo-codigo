# Loom counter model

Run `cargo test -p appendix-loom-model --release --locked` from the parent
workspace. Both tests must pass. One passes because `fetch_add` preserves
both increments. The negative control passes only when Loom finds the
specific `lost increment` assertion failure in the split load/store version.

All modeled threads, atomics and shared ownership use Loom types. The small
model uses sequentially consistent operations to show that a multi-operation
algorithm can be wrong even when each individual operation is atomic.
It does not model the operating system, network I/O or the entire application.

Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
