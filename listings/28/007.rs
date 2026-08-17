#[test]
fn rejects_missing_at() {
    let error = Email::parse("sin-arroba").unwrap_err();
    assert_eq!(error, EmailError::MissingAt);
}
