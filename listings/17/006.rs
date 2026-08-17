trait MinimalIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Countdown(u8);

impl MinimalIterator for Countdown {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let current = self.0;
        (current > 0).then(|| {
            self.0 -= 1;
            current
        })
    }
}

fn main() {
    let mut countdown = Countdown(2);
    assert_eq!(countdown.next(), Some(2));
    assert_eq!(countdown.next(), Some(1));
    assert_eq!(countdown.next(), None);
}
