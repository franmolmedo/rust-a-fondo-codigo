fn print_name(name: &str) {
    println!("{name}");
}

fn main() {
    let owned = String::from("Ada");
    print_name(&owned);
    print_name("Grace");
}
