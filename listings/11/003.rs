fn main() {
    let identity = |value| value;
    let number = identity(10);
    let text = identity(String::from("hola"));
    // error[E0308]: la primera llamada fijó el tipo como i32

    println!("{number} {text}");
}
