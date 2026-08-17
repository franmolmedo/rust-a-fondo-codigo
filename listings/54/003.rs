#[derive(Debug)]
enum CreateUserError {
    InvalidEmail(EmailError),
    DuplicateEmail,
    Storage(StorageError),
}
