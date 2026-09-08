# Chapter 26: a small layered workspace

This standalone workspace accompanies exercises C26-E01 and C26-E07.
It has no external dependencies and does not start an HTTP server.

- `domain` defines an order.
- `application` defines the repository contract and a query.
- `adapters` provides an in-memory implementation.
- `server` assembles the application in a tiny executable.
- `test-support` provides a fake used by the server's integration tests.

The application does not depend on its fake. This avoids a dependency cycle:
the server's tests depend on both the application and test support.

Run these commands from this directory:

```console
cargo run -p server --locked
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo tree -p domain
cargo tree -p server -e normal
cargo tree -p server -e normal,dev
```

The normal server dependency tree does not include `test-support`.
A workspace-wide build can still build that package as an explicitly selected
member; this does not make it a production dependency of the server.

This workspace is intentionally separate from the main code workspace.
Its checks must be run with this manifest or from this directory.
