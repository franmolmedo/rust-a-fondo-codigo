fn greet(name: String) {
    println!("hola, {name}");
}

fn main() {
    let name = String::from("Ada");
    greet(name); // move: `name` se transfiere a greet
    println!("{name}");
    // error[E0382]: borrow of moved value: `name`
}
