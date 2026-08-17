fn main() {
    let values = [1, 2, 3, 4];
    match values.get(10) {
        Some(value) => println!("{value}"),
        None => println!("índice inexistente"),
    }
}
