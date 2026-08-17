fn main() {
    let mut values = vec![String::from("uno")];
    let first = &values[0];

    values.push(String::from("dos"));
    // error[E0502]: the vector is still shared-borrowed

    println!("{first}");
}
