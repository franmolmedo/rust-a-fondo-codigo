#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Email(String);

fn main() {
    let id = UserId(7);
    let copied = id;
    assert_eq!(id, copied);

    let email = Email(String::from("ada@example.test"));
    let cloned = email.clone();
    assert_eq!(email, cloned);
}
