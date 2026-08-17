#[derive(Debug, PartialEq)]
struct User {
    id: u64,
}

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

#[derive(Debug, PartialEq)]
enum LoadUserError {
    Repository { id: u64, source: RepositoryError },
}

fn load_user(
    id: u64,
    repository_result: Result<User, RepositoryError>,
) -> Result<User, LoadUserError> {
    repository_result.map_err(|source| LoadUserError::Repository { id, source })
}

fn main() {
    assert_eq!(
        load_user(7, Err(RepositoryError::Unavailable)),
        Err(LoadUserError::Repository {
            id: 7,
            source: RepositoryError::Unavailable,
        })
    );
}
