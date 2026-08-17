fn main() {
    let maybe_text = Some(String::from("hola"));

    if let Some(_text) = maybe_text {
        println!("hay texto");
    }

    println!("{maybe_text:?}");
    // error[E0382]: value was partially moved
}
