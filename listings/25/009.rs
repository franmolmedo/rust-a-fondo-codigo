mod auth {
    fn hash_password(input: &str) -> String {
        format!("hash:{input}")
    }
}

fn main() {
    auth::hash_password("secreto");
    // error[E0603]: function `hash_password` is private
}
