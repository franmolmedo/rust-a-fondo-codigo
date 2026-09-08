#[derive(Debug, PartialEq)]
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

impl Pair<f64> {
    fn distance_from_origin(&self) -> f64 {
        self.left.hypot(self.right)
    }
}

fn main() {
    let point = Pair::new(3.0_f64, 4.0);
    assert_eq!(point.larger(), Some(&4.0));
    assert_eq!(Pair::new(f64::NAN, 4.0).larger(), None);
    assert_eq!(point.distance_from_origin(), 5.0);

    struct Token;
    let _tokens = Pair::new(Token, Token); // Existe, pero no tiene larger.
}
