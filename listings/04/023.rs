use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("data"));
    assert_eq!(Rc::strong_count(&shared), 1);

    let first = Rc::clone(&shared);
    {
        let second = Rc::clone(&shared);
        assert_eq!(Rc::strong_count(&shared), 3);
        assert_eq!(second.as_str(), "data");
    }

    assert_eq!(Rc::strong_count(&shared), 2);
    drop(first);
    assert_eq!(Rc::strong_count(&shared), 1);
}
