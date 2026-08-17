#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
}

#[derive(Debug, PartialEq)]
enum RepositoryError {
    Unavailable,
}

fn repository_find(
    users: &[User],
    id: u64,
) -> Result<Option<User>, RepositoryError> {
    if id == 0 {
        return Err(RepositoryError::Unavailable);
    }
    Ok(users.iter().find(|user| user.id == id).cloned())
}

#[derive(Debug, PartialEq)]
enum GetUserError {
    NotFound { id: u64 },
    Repository(RepositoryError),
}

fn get_user(users: &[User], id: u64) -> Result<User, GetUserError> {
    repository_find(users, id)
        .map_err(GetUserError::Repository)?
        .ok_or(GetUserError::NotFound { id })
}

fn main() {
    let users = [User { id: 7 }];
    assert_eq!(get_user(&users, 7), Ok(User { id: 7 }));
    assert_eq!(get_user(&users, 8), Err(GetUserError::NotFound { id: 8 }));
    assert_eq!(
        get_user(&users, 0),
        Err(GetUserError::Repository(RepositoryError::Unavailable))
    );
}
