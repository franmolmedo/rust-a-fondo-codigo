#[derive(Debug)]
enum RegisterUserError {
    InvalidEmail,
    DuplicateEmail,
    RepositoryUnavailable,
}

fn into_http(error: RegisterUserError) -> (u16, &'static str) {
    match error {
        RegisterUserError::InvalidEmail => (400, "email inválido"),
        RegisterUserError::DuplicateEmail => (409, "email ya registrado"),
        RegisterUserError::RepositoryUnavailable => {
            (503, "servicio temporalmente no disponible")
        }
    }
}

fn main() {
    assert_eq!(
        into_http(RegisterUserError::DuplicateEmail),
        (409, "email ya registrado")
    );
}
