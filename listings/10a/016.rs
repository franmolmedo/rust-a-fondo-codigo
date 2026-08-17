use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
enum EmailError {
    #[error("el email no puede estar vacío")]
    Empty,
    #[error("el email debe contener @")]
    MissingAt,
    #[error("email demasiado largo: máximo {maximum}, actual {actual}")]
    TooLong { maximum: usize, actual: usize },
}

fn main() {
    let error = EmailError::TooLong {
        maximum: 254,
        actual: 260,
    };
    assert!(error.to_string().contains("260"));
}
