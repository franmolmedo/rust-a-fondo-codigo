# Rust a fondo — supplementary appendix labs

Author: Francisco M. Olmedo Bueno. Code license: MIT.

This workspace accompanies appendices A–G. It is separate from the 403
reference solutions for chapters 1–58: those solutions retain their IDs,
dependencies and verification report. The new labs use Rust 1.95.0; they do
not claim the main corpus's older minimum compiler version.

From this directory, run:

```console
cargo test --workspace --locked
cargo test --workspace --doc --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
python verify.py
```

Python 3.11 or later is only needed for the verification script. Cargo runs
the Rust projects without Python. `verify.py --sync` refreshes the committed
doctests and listing manifest when the book's `docs/` directory is present;
reader copies without the manuscript verify the committed corpus instead.
Reports distinguish commands actually executed from platform-specific labs
that require a separate run. An installer command printed in the book is not
an installation test, and a compiled debugger configuration is not a GUI test.

The workspace contains a first project, standard-library and language labs,
measurement exercises, advanced checks, complete small applications and a
portable `no_std` library. See the individual README files and the book for
their contracts, commands and limitations.

| Appendix | Source | Main verification |
| --- | --- | --- |
| A — setup | [first-steps](first-steps/src/main.rs) | Native run and two unit tests; editor setup is manual. |
| B — standard library | [data.rs](lab/src/data.rs) | Bounded parsing, real error chains and failing writers. |
| C — compile-time evaluation | [const_eval.rs](lab/src/const_eval.rs) | Unit tests and passing/failing doctests. |
| D — debugging/performance | [examples](lab/examples) | Regression tests and an executable measurement experiment. |
| E — advanced tools | [audits.rs](lab/src/audits.rs), [Loom](loom-model/README.md), [C FFI](native-checksum/README.md), [fuzzing](fuzz/README.md) | Positive and negative controls with separate tool reports. |
| F — applications | [inventory-cli](inventory-cli/README.md), [sum-service](sum-service/README.md) | Process and loopback integration tests; [native packaging](package.py). |
| G — no_std | [portable-core](portable-core/README.md) | Host tests, optional allocation and a bare-metal ARM build. |

See [VERIFICATION.md](VERIFICATION.md) for the recorded scope and outstanding
manual checks, and [ci/README.md](ci/README.md) for the GitHub Actions template.
`listings.json` records every appendix fence with an `AP-` ID and source hash;
Rust examples are tested in `lab/doctests.md` or `portable-core/doctests.md`.
Refresh both through `verify.py --sync`, not by editing the generated files.
The 29 appendix questions include their answers in the manuscript and are
separate from the main book's 417 solution cards.

The author developed and reviewed this material with AI assistance. Automated
checks and editorial review do not guarantee that the code is free of errors.
The MIT license covers these code files, not the commercial manuscript.
