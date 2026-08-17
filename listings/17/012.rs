trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct WindowsMut<'slice, T> {
    slice: &'slice mut [T],
    size: usize,
    position: usize,
}

impl<'slice, T> WindowsMut<'slice, T> {
    fn new(slice: &'slice mut [T], size: usize) -> Self {
        Self { slice, size, position: 0 }
    }
}

impl<T> LendingIterator for WindowsMut<'_, T> {
    type Item<'a> = &'a mut [T] where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<&'a mut [T]> {
        let start = self.position;
        let end = start.checked_add(self.size)?;
        if self.size == 0 || end > self.slice.len() {
            return None;
        }
        self.position += 1;
        Some(&mut self.slice[start..end])
    }
}

fn main() {
    let mut data = [1, 2, 3, 4];
    let mut windows = WindowsMut::new(&mut data, 2);

    windows.next().unwrap()[1] = 20; // ventana [0..2]
    windows.next().unwrap()[1] = 30; // ventana [1..3]
    assert_eq!(data, [1, 20, 30, 4]);
}
