use std::io;
use std::net::SocketAddr;
use std::process::ExitCode;
use sum_service::{Limits, serve};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("Usage: sum-server [127.0.0.1:8042]\nLoopback only. Press Ctrl+C to stop.");
        return ExitCode::SUCCESS;
    }
    let address: Option<SocketAddr> = if args.is_empty() {
        "127.0.0.1:8042".parse().ok()
    } else if args.len() == 1 {
        args[0].to_str().and_then(|text| text.parse().ok())
    } else {
        None
    };
    let Some(address) = address.filter(|address| address.ip().is_loopback()) else {
        eprintln!("Usage: sum-server [loopback-address:port]");
        return ExitCode::from(2);
    };
    match run(address).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("sum-server: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run(address: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(address).await?;
    eprintln!("Listening on {}", listener.local_addr()?);
    let stats = serve(listener, tokio::signal::ctrl_c(), Limits::default()).await?;
    eprintln!("Stopped: {stats:?}");
    if stats.panicked != 0 {
        return Err(io::Error::other("a connection task panicked"));
    }
    Ok(())
}
