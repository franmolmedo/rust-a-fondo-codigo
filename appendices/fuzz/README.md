# Parser fuzz target

This is a separate Cargo workspace. From the parent `appendices` directory
on Linux with a C++ compiler installed:

```console
rustup toolchain install nightly-2026-04-15 --profile minimal
cargo +nightly-2026-04-15 install cargo-fuzz --version 0.13.1 --locked
cargo +nightly-2026-04-15 fuzz run parse_record -- -max_total_time=30 -max_len=1024
```

The target checks that accepted records survive parse/serialize/parse and
that processing arbitrary inputs does not panic. It deliberately bounds
input size. This is not a proof of complete format correctness or of resource
limits for the whole importer. Invalid UTF-8 is rejected before text parsing.

Keep a discovered reproducer and turn it into a regression test. Generated
corpora and crash artifacts are ignored by Git; they may contain input data.
Do not publish sensitive reproductions without review. Windows sanitizer
setup has additional requirements; see the Rust Fuzz Book for that platform.

Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
