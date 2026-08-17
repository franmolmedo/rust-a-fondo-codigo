#[test]
fn registration_stores_the_user() {
    let mut repo = InMemoryUsers::default();
    register(&mut repo, User { id: 7, name: String::from("Ada") }).unwrap();
    assert_eq!(repo.find(7).map(|u| u.name.as_str()), Some("Ada"));
}
