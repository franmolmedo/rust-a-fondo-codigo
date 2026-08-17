#[derive(Clone, Debug)]
enum EitherIter<A, B> {
    Forward(A),
    Reverse(B),
}

impl<T, A, B> Iterator for EitherIter<A, B>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            Self::Forward(iterator) => iterator.next(),
            Self::Reverse(iterator) => iterator.next(),
        }
    }
}

fn numbers_static(reverse: bool) -> impl Iterator<Item = u32> {
    if reverse {
        EitherIter::Reverse((0..3).rev())
    } else {
        EitherIter::Forward(0..3)
    }
}

fn numbers_dynamic(reverse: bool) -> Box<dyn Iterator<Item = u32>> {
    if reverse {
        Box::new((0..3).rev())
    } else {
        Box::new(0..3)
    }
}

fn main() {
    assert_eq!(numbers_static(true).collect::<Vec<_>>(), [2, 1, 0]);
    assert_eq!(numbers_dynamic(false).collect::<Vec<_>>(), [0, 1, 2]);
}
