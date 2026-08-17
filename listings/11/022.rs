fn bad_filter() -> impl Fn(&str) -> bool {
    let prefix = String::from("ru");
    |candidate| candidate.starts_with(&prefix)
    // error[E0373]: la closure puede sobrevivir a prefix
}

fn main() {}
