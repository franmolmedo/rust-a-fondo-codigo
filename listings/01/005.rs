fn greet(name: String) {
    println!("hola, {name}");
}

fn main() {
    let name = String::from("Ada");
    greet(name); // `greet` recibe la propiedad de `name`
    println!("{name}");
    // error[E0382]: borrow of moved value: `name`
}
