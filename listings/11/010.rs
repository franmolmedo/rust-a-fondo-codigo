fn main() {
    let text = String::from("owned");
    let consume = move || text;

    let first = consume();
    let second = consume();
    // error[E0382]: use of moved value: consume

    println!("{first} {second}");
}
