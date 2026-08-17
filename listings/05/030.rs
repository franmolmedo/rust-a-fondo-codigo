fn main() {
    let mut count = 0;
    let mut increment = || count += 1;

    println!("{count}");
    // error[E0502]: count is still mutably borrowed by the closure

    increment();
}
