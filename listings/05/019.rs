fn main() {
    let mut value = 10;
    let first = &mut value;
    let second = first;

    *second += 1;
    *first += 1;
    // error[E0382]: first was moved
}
