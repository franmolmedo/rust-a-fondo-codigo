#[derive(Debug, PartialEq)]
struct User(&'static str);

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

fn find_user(id: u64) -> Result<Option<User>, RepositoryError> {
    match id {
        7 => Ok(Some(User("Ada"))),
        0 => Err(RepositoryError::Unavailable),
        _ => Ok(None),
    }
}

fn main() {
    assert_eq!(find_user(7), Ok(Some(User("Ada"))));
    assert_eq!(find_user(9), Ok(None));
    assert_eq!(find_user(0), Err(RepositoryError::Unavailable));
}
