//! Bounded, loopback-only, one-request-per-connection TCP service.

use rust_appendix_lab::protocol::{Reply, evaluate};
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinError, JoinSet};
use tokio::time::{sleep, timeout};

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_connections: usize,
    pub request_bytes: usize,
    pub request_timeout: Duration,
    pub shutdown_grace: Duration,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_connections: 16,
            request_bytes: 128,
            request_timeout: Duration::from_secs(5),
            shutdown_grace: Duration::from_secs(2),
        }
    }
}

impl Limits {
    fn validate(self) -> io::Result<()> {
        if !(1..=1024).contains(&self.max_connections)
            || !(1..=4096).contains(&self.request_bytes)
            || self.request_timeout.is_zero()
            || self.request_timeout > Duration::from_secs(3600)
            || self.shutdown_grace.is_zero()
            || self.shutdown_grace > Duration::from_secs(3600)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid resource limits",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Stats {
    pub accepted: u64,
    pub rejected: u64,
    pub completed: u64,
    pub timed_out: u64,
    pub io_failed: u64,
    pub cancelled: u64,
    pub panicked: u64,
    pub high_water: usize,
}

enum Outcome {
    Completed,
    TimedOut,
    IoFailed,
}

fn record(stats: &mut Stats, result: Result<Outcome, JoinError>) {
    let counter = match result {
        Ok(Outcome::Completed) => &mut stats.completed,
        Ok(Outcome::TimedOut) => &mut stats.timed_out,
        Ok(Outcome::IoFailed) => &mut stats.io_failed,
        Err(error) if error.is_cancelled() => &mut stats.cancelled,
        Err(_) => &mut stats.panicked,
    };
    *counter = counter.saturating_add(1);
}

/// Read one LF-terminated frame, with at most limit+1 bytes consumed.
/// Any read-ahead is discarded: this helper is for this one-request protocol.
pub async fn read_frame(
    reader: &mut (impl AsyncRead + Unpin),
    limit: usize,
) -> io::Result<Vec<u8>> {
    if !(1..=4096).contains(&limit) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid frame limit",
        ));
    }
    let mut limited = BufReader::with_capacity(limit + 1, reader.take((limit + 1) as u64));
    let mut frame = Vec::with_capacity(limit + 1);
    limited.read_until(b'\n', &mut frame).await?;
    if frame.len() > limit || !frame.ends_with(b"\n") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "frame too long or missing LF",
        ));
    }
    Ok(frame)
}

async fn handle(mut stream: TcpStream, limits: Limits) -> io::Result<()> {
    stream.write_all(b"READY\n").await?;
    let reply = match read_frame(&mut stream, limits.request_bytes).await {
        Ok(frame) => evaluate(&frame),
        Err(error) if error.kind() == io::ErrorKind::InvalidData => Reply::InvalidRequest,
        Err(error) => return Err(error),
    };
    stream.write_all(reply.to_line().as_bytes()).await?;
    stream.shutdown().await
}

/// Stop accepting when shutdown resolves, drain until the grace deadline,
/// then abort and join all remaining connection tasks. No tasks are detached.
pub async fn serve(
    listener: TcpListener,
    shutdown: impl Future<Output = io::Result<()>>,
    limits: Limits,
) -> io::Result<Stats> {
    limits.validate()?;
    if !listener.local_addr()?.ip().is_loopback() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only loopback listeners are allowed",
        ));
    }
    let mut tasks = JoinSet::new();
    let mut stats = Stats::default();
    tokio::pin!(shutdown);
    let terminal_error = loop {
        tokio::select! {
            biased;
            result = &mut shutdown => break result.err(),
            Some(result) = tasks.join_next(), if !tasks.is_empty() => record(&mut stats, result),
            incoming = listener.accept() => {
                let (stream, _) = match incoming {
                    Ok(pair) => pair,
                    Err(error) => break Some(error),
                };
                if tasks.len() >= limits.max_connections {
                    stats.rejected = stats.rejected.saturating_add(1);
                    drop(stream);
                    continue;
                }
                stats.accepted = stats.accepted.saturating_add(1);
                tasks.spawn(async move {
                    match timeout(limits.request_timeout, handle(stream, limits)).await {
                        Ok(Ok(())) => Outcome::Completed,
                        Ok(Err(_)) => Outcome::IoFailed,
                        Err(_) => Outcome::TimedOut,
                    }
                });
                stats.high_water = stats.high_water.max(tasks.len());
            }
        }
    };
    drop(listener);
    let deadline = sleep(limits.shutdown_grace);
    tokio::pin!(deadline);
    while !tasks.is_empty() {
        tokio::select! {
            biased;
            _ = &mut deadline => {
                tasks.abort_all();
                while let Some(result) = tasks.join_next().await { record(&mut stats, result); }
                break;
            }
            Some(result) = tasks.join_next() => record(&mut stats, result),
        }
    }
    match terminal_error {
        Some(error) => Err(error),
        None => Ok(stats),
    }
}

/// The timeout covers connection, greeting, request and reply together.
pub async fn request(address: SocketAddr, left: u64, right: u64) -> io::Result<String> {
    if !address.ip().is_loopback() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only loopback addresses are allowed",
        ));
    }
    timeout(Duration::from_secs(5), async {
        let mut stream = TcpStream::connect(address).await?;
        if read_frame(&mut stream, 32).await? != b"READY\n" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected greeting",
            ));
        }
        stream
            .write_all(format!("SUM {left} {right}\n").as_bytes())
            .await?;
        let reply = read_frame(&mut stream, 64).await?;
        let expected = evaluate(format!("SUM {left} {right}\n").as_bytes()).to_line();
        if reply != expected.as_bytes() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected reply",
            ));
        }
        Ok(expected)
    })
    .await
    .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "request deadline expired"))?
}
