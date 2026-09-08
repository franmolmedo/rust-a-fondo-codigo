use std::path::Path;
use std::process::Command;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(name)
}

#[test]
fn executable_reads_a_real_file() {
    let output = Command::new(env!("CARGO_BIN_EXE_inventory-cli"))
        .arg(fixture("inventory.csv"))
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"adaptador,4\ncable,7\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_input_produces_no_partial_report() {
    let output = Command::new(env!("CARGO_BIN_EXE_inventory-cli"))
        .arg(fixture("invalid.csv"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("line 2"));
    assert!(error.contains("caused by:"));
}

#[test]
fn missing_arguments_and_help_have_distinct_statuses() {
    let missing = Command::new(env!("CARGO_BIN_EXE_inventory-cli"))
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(2));
    let help = Command::new(env!("CARGO_BIN_EXE_inventory-cli"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(help.stdout.starts_with(b"Usage:"));
    assert!(help.stderr.is_empty());
}

#[test]
fn missing_file_is_an_operational_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_inventory-cli"))
        .arg(fixture("not-an-input-file.csv"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
}
