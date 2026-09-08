mod percentages {
    pub struct Percentage(u8);
}

fn main() {
    let p = percentages::Percentage(150);
}
// error[E0603]: cannot initialize a tuple struct which contains private fields
