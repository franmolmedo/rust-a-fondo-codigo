#[derive(Debug, PartialEq, Eq)]
struct Email(String);

#[derive(Debug, PartialEq, Eq)]
struct User {
    email: Email,
}

trait Notifier {
    fn send_raw(&mut self, to: &Email, body: &str);

    fn welcome(&mut self, user: &User) {
        self.send_raw(&user.email, "Bienvenido");
    }

    fn password_reset(&mut self, user: &User, token: &str) {
        self.send_raw(&user.email, &format!("Token: {token}"));
    }
}

#[derive(Default)]
struct RecordingNotifier {
    sent: Vec<String>,
}

impl Notifier for RecordingNotifier {
    fn send_raw(&mut self, to: &Email, body: &str) {
        self.sent.push(format!("{}: {body}", to.0));
    }
}

fn main() {
    let user = User { email: Email(String::from("ada@example.com")) };
    let mut notifier = RecordingNotifier::default();
    notifier.welcome(&user);
    notifier.password_reset(&user, "abc");
    assert_eq!(notifier.sent.len(), 2);
}
