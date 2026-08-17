fn main() {
    let mut text = String::from("hola");
    let read = &text;
    let write = &mut text;
    // error[E0502]: shared and mutable borrows overlap

    write.push('!');
    println!("{read}");
}
