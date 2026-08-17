fn main() {
    let first = String::from("hola");
    let second = first;

    println!("{second}");
    println!("{first}");
    // error[E0382]: borrow of moved value: `first`
}
