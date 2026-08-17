use catalog::Percentage;

#[test]
fn public_constructor_is_usable() {
    assert_eq!(Percentage::new(25).unwrap().get(), 25);
}
