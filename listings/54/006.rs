#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let users = vec![
        User { name: " Ada ".into() },
        User { name: "  ".into() },
    ];

    let borrowed: Vec<&str> = users
        .iter()
        .map(|user| user.name.trim())
        .filter(|name| !name.is_empty())
        .collect();
    assert_eq!(borrowed, ["Ada"]);

    let owned: Vec<String> = borrowed.into_iter().map(str::to_owned).collect();
    assert_eq!(owned, ["Ada"]);
}
