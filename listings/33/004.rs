fn main() {
    let future = async {
        println!("body executed");
        42
    };

    // No one polls this future, so its body never runs.
    drop(future);
    println!("program finished");
}
