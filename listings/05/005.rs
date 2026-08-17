fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut message = String::from("hola");
    add_exclamation(&mut message);
    add_exclamation(&mut message);

    assert_eq!(message, "hola!!");
}
