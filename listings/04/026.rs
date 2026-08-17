fn main() {
    let text = String::from("hola");
    let print = move || println!("{text}");

    println!("{text}");
    print();
    // error[E0382]: borrow of moved value: `text`
}
