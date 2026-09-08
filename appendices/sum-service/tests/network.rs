use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use sum_service::{Limits, Stats, read_frame, request, serve};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::timeout;

struct Server {
    address: SocketAddr,
    stop: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<io::Result<Stats>>>,
}

impl Server {
    async fn start(limits: Limits) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (stop, receiver) = oneshot::channel();
        let task = tokio::spawn(serve(
            listener,
            async {
                receiver
                    .await
                    .map_err(|_| io::Error::other("shutdown sender dropped"))
            },
            limits,
        ));
        Self {
            address,
            stop: Some(stop),
            task: Some(task),
        }
    }

    async fn connected(&self) -> TcpStream {
        let mut stream = TcpStream::connect(self.address).await.unwrap();
        assert_eq!(
            timeout(Duration::from_secs(5), read_frame(&mut stream, 32))
                .await
                .unwrap()
                .unwrap(),
            b"READY\n"
        );
        stream
    }

    async fn finish(mut self) -> Stats {
        self.stop.take().unwrap().send(()).unwrap();
        timeout(Duration::from_secs(5), self.task.take().unwrap())
            .await
            .unwrap()
            .unwrap()
            .unwrap()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

#[tokio::test]
async fn real_tcp_round_trip_and_overflow() {
    let server = Server::start(Limits::default()).await;
    assert_eq!(request(server.address, 19, 23).await.unwrap(), "OK 42\n");
    assert_eq!(
        request(server.address, u64::MAX, 1).await.unwrap(),
        "ERR overflow\n"
    );
    let stats = server.finish().await;
    assert_eq!(stats.completed, 2);
    assert_eq!(stats.accepted, 2);
    assert_eq!(stats.panicked, 0);
}

#[tokio::test]
async fn byte_limit_rejects_an_unterminated_request() {
    let server = Server::start(Limits::default()).await;
    let mut stream = server.connected().await;
    stream.write_all(&[b'x'; 129]).await.unwrap();
    assert_eq!(
        timeout(Duration::from_secs(5), read_frame(&mut stream, 64))
            .await
            .unwrap()
            .unwrap(),
        b"ERR invalid request\n"
    );
    assert_eq!(server.finish().await.completed, 1);
}

#[tokio::test]
async fn truncated_and_invalid_utf8_requests_have_explicit_replies() {
    let server = Server::start(Limits::default()).await;
    for bytes in [b"SUM 1".as_slice(), b"SUM 1 \xff\n"] {
        let mut stream = server.connected().await;
        stream.write_all(bytes).await.unwrap();
        stream.shutdown().await.unwrap();
        assert_eq!(
            timeout(Duration::from_secs(5), read_frame(&mut stream, 64))
                .await
                .unwrap()
                .unwrap(),
            b"ERR invalid request\n"
        );
    }
    assert_eq!(server.finish().await.completed, 2);
}

#[tokio::test]
async fn admission_limit_does_not_spawn_an_extra_task() {
    let server = Server::start(Limits {
        max_connections: 1,
        ..Limits::default()
    })
    .await;
    let mut admitted = server.connected().await;
    let mut rejected = TcpStream::connect(server.address).await.unwrap();
    let mut byte = [0];
    let read = timeout(Duration::from_secs(5), rejected.read(&mut byte))
        .await
        .unwrap();
    assert!(matches!(read, Ok(0)) || read.is_err());
    admitted.write_all(b"SUM 1 2\n").await.unwrap();
    assert_eq!(read_frame(&mut admitted, 64).await.unwrap(), b"OK 3\n");
    let stats = server.finish().await;
    assert_eq!(stats.high_water, 1);
    assert_eq!(stats.accepted, 1);
    assert_eq!(stats.rejected, 1);
}

#[tokio::test]
async fn idle_client_hits_the_request_deadline() {
    let server = Server::start(Limits {
        request_timeout: Duration::from_millis(250),
        ..Limits::default()
    })
    .await;
    let mut stream = server.connected().await;
    let mut byte = [0];
    assert_eq!(
        timeout(Duration::from_secs(5), stream.read(&mut byte))
            .await
            .unwrap()
            .unwrap(),
        0
    );
    assert_eq!(server.finish().await.timed_out, 1);
}

#[tokio::test]
async fn shutdown_cancels_and_joins_an_idle_task_after_grace() {
    let server = Server::start(Limits {
        request_timeout: Duration::from_secs(60),
        shutdown_grace: Duration::from_millis(30),
        ..Limits::default()
    })
    .await;
    let mut stream = server.connected().await;
    let stats = server.finish().await;
    assert_eq!(stats.cancelled, 1);
    let mut byte = [0];
    assert_eq!(stream.read(&mut byte).await.unwrap(), 0);
}

#[tokio::test]
async fn shutdown_allows_an_admitted_request_to_finish() {
    let mut server = Server::start(Limits::default()).await;
    let mut stream = server.connected().await;
    server.stop.take().unwrap().send(()).unwrap();
    stream.write_all(b"SUM 7 8\n").await.unwrap();
    assert_eq!(read_frame(&mut stream, 64).await.unwrap(), b"OK 15\n");
    let stats = timeout(Duration::from_secs(5), server.task.take().unwrap())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.cancelled, 0);
}

#[tokio::test]
async fn invalid_configuration_is_rejected_before_accepting() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let error = serve(
        listener,
        std::future::pending(),
        Limits {
            request_bytes: 0,
            ..Limits::default()
        },
    )
    .await
    .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[tokio::test]
async fn compiled_client_talks_to_the_running_service() {
    let server = Server::start(Limits::default()).await;
    let address = server.address.to_string();
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new(env!("CARGO_BIN_EXE_sum-client"))
            .args(["19", "23", &address])
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"OK 42\n");
    assert_eq!(server.finish().await.completed, 1);
}

#[test]
fn server_executable_has_help_and_rejects_public_binding() {
    let help = std::process::Command::new(env!("CARGO_BIN_EXE_sum-server"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    let invalid = std::process::Command::new(env!("CARGO_BIN_EXE_sum-server"))
        .arg("0.0.0.0:8042")
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(2));
}

#[tokio::test]
async fn frame_reader_handles_short_chunks_and_exact_limits() {
    let (mut writer, mut reader) = tokio::io::duplex(1);
    let sender = tokio::spawn(async move {
        writer.write_all(b"ab\n").await.unwrap();
    });
    assert_eq!(read_frame(&mut reader, 3).await.unwrap(), b"ab\n");
    sender.await.unwrap();
}
