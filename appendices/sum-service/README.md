# Bounded sum service

Two complete executables, a transport library, a pure protocol parser in the
shared lab, and real loopback integration tests. From the parent workspace:

```console
cargo run -p sum-service --bin sum-server --locked
cargo run -p sum-service --bin sum-client --locked -- 19 23
cargo test -p sum-service --locked
cargo build -p sum-service --bins --release --locked
```

Run the server and client in separate terminals. Default address:
`127.0.0.1:8042`. Optional final address argument on both executables.
Only numeric loopback addresses are allowed. Ctrl+C stops the server.

Protocol: server sends `READY\n` after admission; client sends exactly
`SUM <left> <right>\n`, using unsigned decimal `u64` operands and single
spaces. CRLF requests and leading zeroes are accepted. One request and reply
per connection. Responses: `OK <sum>\n`, `ERR overflow\n`, or
`ERR invalid request\n`. Unknown/malformed server responses are rejected by
the provided client. Client exits: 0 for OK, 1 for protocol/transport failure,
2 for bad arguments. The server reports startup and aggregate shutdown stats
on stderr, without logging request bodies.

Default resource limits: 16 supervised tasks, 128 request bytes including LF,
five seconds for the entire admitted connection, and two seconds shutdown
grace. A full service closes excess accepted sockets without starting more
tasks. The OS socket backlog is separate. The bounded frame reader may
consume one extra byte to detect overflow. This is NOT a persistent framing
API: any read-ahead is discarded when the one-request connection ends.

Shutdown stops acceptance, lets existing tasks finish within a single grace
deadline, aborts the remainder, then joins them. Cancellation does not roll
back remote effects. These handlers only calculate a number, which keeps
the cancellation policy simple. A timeout cannot preempt arbitrary blocking
or CPU-bound code; keep such work out of these handlers.

No TLS, authentication, persistence, automatic retries or public deployment.
Do not expose this teaching service to the Internet. Tests use ephemeral
loopback ports and test requests, malformed input, limits, deadlines, graceful
completion, cancellation and the actual client executable. Manual Ctrl+C
delivery is a separate platform/UI check.

Ship both native release executables with this README and the workspace MIT
LICENSE. Verify each OS build on a clean machine before public distribution.
Code: MIT. Author: Francisco M. Olmedo Bueno. Developed with AI assistance.
