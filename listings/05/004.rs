fn main() {
    let names = vec![String::from("Ada")];
    let view = &names[0];

    let owned: String = *view;
    // error[E0507]: cannot move out through a shared reference

    println!("{owned}");
}
