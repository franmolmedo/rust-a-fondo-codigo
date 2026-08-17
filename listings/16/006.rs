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
    fn larger(&self) -> &T {
        if self.left >= self.right { &self.left } else { &self.right }
    }
}

impl Pair<f64> {
    fn distance_from_origin(&self) -> f64 {
        self.left.hypot(self.right)
    }
}

fn main() {
    let point = Pair::new(3.0_f64, 4.0);
    assert_eq!(point.larger(), &4.0);
    assert_eq!(point.distance_from_origin(), 5.0);

    struct Token;
    let _tokens = Pair::new(Token, Token); // Existe, pero no tiene larger.
}
