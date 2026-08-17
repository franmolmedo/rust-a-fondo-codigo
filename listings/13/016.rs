fn normalized_words() -> impl Iterator<Item = String> {
    let data = vec![String::from(" hola "), String::from(" mundo ")];
    data.iter().map(|value| value.trim().to_owned())
    // error[E0515]: el iterador contiene un préstamo de data
}

fn main() {}
