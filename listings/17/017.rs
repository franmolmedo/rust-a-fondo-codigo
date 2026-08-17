trait ViewStore {
    type View<'a> where Self: 'a;
    fn view(&self) -> Self::View<'_>;
}

fn erase(_store: &dyn ViewStore) {}

fn main() {}
