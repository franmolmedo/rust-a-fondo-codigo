fn main() {
    let mut names = vec![String::from("Ada"), String::from("Grace")];

    let observed: Vec<&str> = names.iter().map(String::as_str).collect();
    assert_eq!(observed, ["Ada", "Grace"]);

    names.iter_mut().for_each(|name| name.make_ascii_uppercase());
    assert_eq!(names, ["ADA", "GRACE"]);

    let lengths: Vec<usize> = names.into_iter().map(|name| name.len()).collect();
    assert_eq!(lengths, [3, 5]);
}
