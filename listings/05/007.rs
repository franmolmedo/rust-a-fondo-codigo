fn main() {
    let mut message = String::from("hola");
    let edit = &mut message;

    println!("{message}");
    // error[E0502]: shared access conflicts with the mutable borrow

    edit.push('!');
}
