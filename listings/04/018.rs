struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Ada"),
        age: 36,
    };

    let name = person.name;
    assert_eq!(name, "Ada");
    assert_eq!(person.age, 36);
}
