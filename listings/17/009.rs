trait ViewStore {
    type View<'a>
    where
        Self: 'a;

    fn view(&self, index: usize) -> Option<Self::View<'_>>;
}

struct Names(Vec<String>);

impl ViewStore for Names {
    type View<'a> = &'a str where Self: 'a;

    fn view(&self, index: usize) -> Option<&str> {
        self.0.get(index).map(String::as_str)
    }
}

fn main() {
    let names = Names(vec![String::from("Ada"), String::from("Grace")]);
    assert_eq!(names.view(1), Some("Grace"));
}
