#[test]
fn rejects_missing_at() {
    let error = Email::parse("missing-at").unwrap_err();
    assert_eq!(error, EmailError::MissingAt);
}
