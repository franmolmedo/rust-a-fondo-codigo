#[derive(Debug, PartialEq)]
struct Email(String);

struct User {
    email: Email,
    active: bool,
}

impl User {
    fn email(&self) -> &Email {
        &self.email
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn replace_email(&mut self, email: Email) -> Email {
        std::mem::replace(&mut self.email, email)
    }

    fn into_email(self) -> Email {
        self.email
    }
}

fn main() {
    let mut user = User {
        email: Email(String::from("old@example.test")),
        active: true,
    };

    assert_eq!(user.email().0, "old@example.test");
    assert!(user.is_active());
    let old = user.replace_email(Email(String::from("new@example.test")));
    assert_eq!(old.0, "old@example.test");
    assert_eq!(user.into_email().0, "new@example.test");
}
