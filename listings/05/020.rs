fn main() {
    let mut values = [10, 20, 30];
    let first = &mut values[0];
    let second = &mut values[1];
    // error[E0499]: indexed mutable borrows may overlap

    *first += 1;
    *second += 1;
}
