use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum LoadError {
    NotFound { key: String },
    InvalidFormat { line: usize },
    Backend { source: Box<dyn Error + Send + Sync> },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { key } => write!(f, "key not found: {key}"),
            Self::InvalidFormat { line } => write!(f, "invalid format at line {line}"),
            Self::Backend { .. } => f.write_str("storage backend failed"),
        }
    }
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Backend { source } => Some(source.as_ref()),
            _ => None,
        }
    }
}

fn main() {
    let error = LoadError::NotFound { key: "port".into() };
    assert!(matches!(error, LoadError::NotFound { .. }));
}
