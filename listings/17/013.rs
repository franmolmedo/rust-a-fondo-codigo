trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct OneAtATime<'a>(&'a mut [u8]);

impl LendingIterator for OneAtATime<'_> {
    type Item<'a> = &'a mut u8 where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<&'a mut u8> {
        self.0.first_mut()
    }
}

fn main() {
    let mut data = [1];
    let mut lender = OneAtATime(&mut data);
    let first = lender.next().unwrap();
    let second = lender.next().unwrap();
    *first += *second;
    // error[E0499]: lender sigue prestado por first
}
