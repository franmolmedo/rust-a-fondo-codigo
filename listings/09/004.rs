fn main() {
    let maybe_value = Some(10);
    let Some(value) = maybe_value;
    // error[E0005]: refutable pattern in local binding

    println!("{value}");
}
