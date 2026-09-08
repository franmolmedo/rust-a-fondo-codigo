use std::io::{self, Read};

#[derive(Debug)]
enum LoadConfigError {
    Io(io::Error),
}

fn load_config_from(mut reader: impl Read) -> Result<String, LoadConfigError> {
    let mut text = String::new();
    reader.read_to_string(&mut text).map_err(LoadConfigError::Io)?;
    Ok(text)
}

struct DeniedReader;

impl Read for DeniedReader {
    fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "read denied"))
    }
}

#[test]
fn io_failure_preserves_the_error_kind() {
    let error = load_config_from(DeniedReader).unwrap_err();
    assert!(matches!(error, LoadConfigError::Io(source)
        if source.kind() == io::ErrorKind::PermissionDenied));
}
