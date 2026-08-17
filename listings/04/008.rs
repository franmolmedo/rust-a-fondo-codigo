fn consume(text: String) {
    println!("{text}");
}

fn main() {
    let name = String::from("Ada");
    consume(name);
    println!("{name}");
    // error[E0382]: borrow of moved value: `name`
}
