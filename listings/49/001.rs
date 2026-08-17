macro_rules! vec_of_strings {
    ($($value:expr),* $(,)?) => {{
        let mut output = Vec::new();
        $(output.push($value.to_string());)*
        output
    }};
}

let names = vec_of_strings!["Ada", String::from("Grace"),];
assert_eq!(names, ["Ada", "Grace"]);
