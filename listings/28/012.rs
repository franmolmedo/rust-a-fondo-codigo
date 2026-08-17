struct UnavailableUsers;

impl UserRepository for UnavailableUsers {
    fn add(&mut self, _user: User) -> Result<(), RepoError> {
        Err(RepoError::Unavailable)
    }

    fn find(&self, _id: u64) -> Option<&User> {
        None
    }
}

#[test]
fn registration_reports_backend_failure() {
    let mut repo = UnavailableUsers;
    let error = register(&mut repo, User { id: 7, name: String::from("Ada") }).unwrap_err();
    assert!(matches!(error, RegisterError::Repository(RepoError::Unavailable)));
}
