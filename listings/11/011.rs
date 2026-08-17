fn call_twice<F>(operation: F)
where
    F: Fn(),
{
    operation();
    operation();
}

fn main() {
    let text = String::from("hola");
    let print_length = move || println!("{}", text.len());

    call_twice(print_length);
}
