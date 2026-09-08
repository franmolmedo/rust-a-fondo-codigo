# Inventory CLI

A complete bounded UTF-8 importer with a real executable, input fixtures,
process-level tests and explicit exit statuses. From the parent workspace:

```console
cargo run -p inventory-cli --locked -- inventory-cli/fixtures/inventory.csv
cargo test -p inventory-cli --locked
cargo build -p inventory-cli --release --locked
```

Input: one `name,quantity` record per line, without headers or quoted CSV
fields. Names contain 1–64 UTF-8 bytes, no comma or control characters.
Surrounding whitespace is trimmed; quantities must fit in `u64`. CRLF,
LF and an unterminated final line are accepted. Limits: 256 bytes per line
including terminators, 1 MiB total input and 1024 distinct names.

Output: sorted totals on stdout, diagnostics on stderr. Exit status 0 means
success, 1 means an input/I/O failure, and 2 means incorrect invocation.
All input is validated before writing the report; writing to an external
destination can still fail halfway. The program itself never overwrites a
file. Shell redirection can overwrite files: never redirect to your input.
PowerShell versions may also differ in their redirection encoding.

Ship the release executable (`.exe` on Windows), this README, the workspace
MIT LICENSE and sample input. Compile on each supported platform and test on
a clean machine. A Linux executable is not a Windows executable; C runtime
and system-library requirements still depend on the build target.

Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
