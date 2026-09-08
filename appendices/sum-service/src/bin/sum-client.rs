use std::net::SocketAddr;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--help" {
        println!("Usage: sum-client <left-u64> <right-u64> [127.0.0.1:8042]");
        return ExitCode::SUCCESS;
    }
    let number = |index: usize| -> Option<u64> {
        let value = args.get(index)?.to_str()?;
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        value.parse().ok()
    };
    let address: Option<SocketAddr> = match args.get(2) {
        Some(value) => value.to_str().and_then(|text| text.parse().ok()),
        None => "127.0.0.1:8042".parse().ok(),
    };
    let (Some(left), Some(right), Some(address)) = (number(0), number(1), address) else {
        eprintln!("Usage: sum-client <left-u64> <right-u64> [loopback-address:port]");
        return ExitCode::from(2);
    };
    if !(2..=3).contains(&args.len()) || !address.ip().is_loopback() {
        eprintln!("Usage: sum-client <left-u64> <right-u64> [loopback-address:port]");
        return ExitCode::from(2);
    }
    match sum_service::request(address, left, right).await {
        Ok(reply) => {
            print!("{reply}");
            if reply.starts_with("OK ") {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            eprintln!("sum-client: {error}");
            ExitCode::FAILURE
        }
    }
}
