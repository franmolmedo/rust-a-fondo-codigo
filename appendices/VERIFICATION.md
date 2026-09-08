# Appendix verification record

Editorial checks recorded on 2026-09-07. Native checks repeated in the
standalone public-code checkout on 2026-09-08, without the manuscript.
This report covers the supplementary A–G workspace, not the main book's
892 listings and 403 reference solutions.

## Public-code synchronization (v2)

All nine commands in `python appendices/verify.py` passed on Windows 11 with
Rust 1.95.0. This includes native tests, doctests, release tests, Clippy,
optional-allocation checks, and compilation of the fuzz target. The 78
committed listings and both doctest corpora passed their hash checks.

The repository now includes the labs and a code-only
[GitHub Actions workflow](../.github/workflows/appendices.yml). Consult its
run results for remote verification; local Windows checks alone do not
establish that the Linux, Miri, fuzzing, or cross-target jobs have passed.
The editorial checks below are the earlier, separately recorded results.

## Source coverage

- Seven appendix manuscripts, with 29 questions and their answers.
- 78 fenced listings recorded in `listings.json` with source locations and hashes.
- 33 Rust blocks: 29 ordinary examples, three expected compilation failures
  and one example that intentionally panics. There are no ignored Rust blocks.
- The allocation example in appendix G is conditional. Both the default and
  `alloc`-enabled doctest suites were executed, so the enabled body is tested.
- 54 declared unit/integration tests across the source packages, including one
  additional allocation-feature test. Native default configuration runs 53;
  the feature-enabled portable package runs its six tests separately.
- Tests in different profiles, Miri and feature configurations are repeat runs,
  not additional distinct test cases.

## Checks executed successfully

Host: Windows 11 x86_64, Rust 1.95.0 (2026-04-14), Python 3.12.1.

| Check | Observed result |
| --- | --- |
| `python verify.py` | Formatting, native workspace tests, doctests, Clippy with warnings denied, release tests, allocation-feature tests, and fuzz-target format/type checks passed. |
| `python verify.py --check-docs` | The committed listing manifest and both doctest corpora match the manuscript. |
| Native C FFI | The C source compiled with the MSVC toolchain, linked, and was called by two passing tests. |
| Loom | The atomic increment model passed; the negative control detected the expected lost increment. |
| `python advanced.py --miri` | Three Rust-only memory tests passed on nightly-2026-04-15; the isolated negative control was rejected for an out-of-bounds read. |
| `python advanced.py --cross` | The library built for `thumbv7em-none-eabihf`, both without allocation and with `alloc`. These are library builds, not board executions. |
| Application tests | Four CLI process tests and eleven service tests passed, including real loopback I/O, the compiled client, admission limits, deadlines and both shutdown paths. |
| Native packaging | A Windows ZIP was built with all three executables, instructions, sample input, license files and hashes. Its executables passed help checks. The script refuses to overwrite that package. |
| Performance experiment | The release executable ran with 4096 values, 2048 queries and 1024 matches, collecting nine samples per strategy. Numbers are local observations, not portable performance promises. |
| Editorial integration | The PDF and EPUB source selectors both include 68 documents and the same seven appendix titles in order. All 27 internal links from appendix pages resolve after rendering. |
| Minimal F reconstruction | A temporary workspace built from the manuscript's minimal manifest, only `data`/`protocol`, and the two application packages passed `cargo test --workspace --offline`. |
| Main-book validation | The existing solution cards, listing references, glossary, diagrams and relative links remain valid. |

Machine-readable local reports are generated under `reports/` and ignored by
Git. Reproduce them from this directory with:

```console
python verify.py
python verify.py --check-docs
python advanced.py --miri
python advanced.py --cross
```

The last two commands require the separately documented nightly component and
ARM target. From the editorial repository root, also run:

```console
python tools/validate_book.py
python tools/check_appendix_integration.py
```

The integration check needs the manuscript and installed book-rendering Python
dependencies; reader code copies do not need it. It creates and removes only
its own temporary reconstruction directory. Cargo build caches stay under the
appendix workspace's ignored `target/reconstruction` directory.

## Not executed or not established

- Fresh Windows/Linux installation procedures and interactive Zed debugging.
- Native Linux execution, a Linux sanitizer-instrumented fuzzing campaign,
  profiling-tool capture, and remote GitHub Actions execution. The fuzz target
  compiled on Windows, which does not establish instrumented fuzzing coverage.
- Manual Ctrl+C delivery in every terminal/platform, clean-machine installation,
  signing, public deployment, physical device operation or real-time guarantees.
- Book publication and KDP upload are outside this code repository. The
  commercial manuscript, PDF and EPUB are intentionally not included.

The source review and these checks are complementary. They do not prove the
absence of defects, complete Rust expertise, or compatibility with every OS,
editor version and hardware target.
