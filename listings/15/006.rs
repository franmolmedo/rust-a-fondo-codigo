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
    fn larger(&self) -> &T {
        if self.left >= self.right { &self.left } else { &self.right }
    }
}

fn main() {
    let numbers = Pair::new(10, 30);
    assert_eq!(numbers.larger(), &30);

    struct Token;
    let tokens = Pair::new(Token, Token);
    let _ = tokens; // El tipo existe; simplemente no ofrece larger.
}
