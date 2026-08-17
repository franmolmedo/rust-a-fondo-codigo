fn greet(name: &str) {
    println!("hola, {name}");
}

fn main() {
    let name = String::from("Ada");
    greet(&name);
    println!("{name}"); // sigue disponible: nadie lo consumió
}
