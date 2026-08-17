fn main() {
    let mut names = vec![String::from("Ada"), String::from("Grace")];

    for name in &names {
        assert!(!name.is_empty());
    }

    for name in &mut names {
        name.push('!');
    }

    let owned: Vec<String> = names.into_iter().collect();
    assert_eq!(owned, [String::from("Ada!"), String::from("Grace!")]);
}
