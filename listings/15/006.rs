#[derive(Debug, PartialEq, Eq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> Option<&T> {
        use std::cmp::Ordering;
        match self.left.partial_cmp(&self.right)? {
            Ordering::Less => Some(&self.right),
            Ordering::Equal | Ordering::Greater => Some(&self.left),
        }
    }
}

fn main() {
    let numbers = Pair::new(10, 30);
    assert_eq!(numbers.larger(), Some(&30));
    assert_eq!(Pair::new(f64::NAN, 1.0).larger(), None);

    struct Token;
    let tokens = Pair::new(Token, Token);
    let _ = tokens; // El tipo existe; simplemente no ofrece larger.
}
