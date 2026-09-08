use rust_appendix_lab::data::{Limits, summarize, write_report};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "Usage: inventory-cli <input-file>\nReads bounded UTF-8 name,quantity records; writes sorted totals to stdout.\n";

fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open(path)?);
    let totals = summarize(input, Limits::default())?;
    let output = BufWriter::new(io::stdout().lock());
    write_report(output, &totals)?;
    Ok(())
}

fn main() -> ExitCode {
    let mut arguments = std::env::args_os().skip(1);
    let Some(path) = arguments.next() else {
        eprint!("{USAGE}");
        return ExitCode::from(2);
    };
    if arguments.next().is_some() {
        eprint!("{USAGE}");
        return ExitCode::from(2);
    }
    if path == OsStr::new("--help") {
        return match io::stdout().lock().write_all(USAGE.as_bytes()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        };
    }
    match run(Path::new(&path)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("inventory-cli: {error}");
            let mut source = error.source();
            while let Some(cause) = source {
                eprintln!("  caused by: {cause}");
                source = cause.source();
            }
            ExitCode::FAILURE
        }
    }
}
