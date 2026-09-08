use course_solutions::abstraction::c17::{LendingIterator, WindowsMut, total_text_len};

#[test]
fn c17_text_lengths_count_bytes_and_accept_empty_input() {
    assert_eq!(total_text_len([]), Some(0));
    assert_eq!(total_text_len(["ñ", "Rust"]), Some(6));
}

#[test]
fn c17_windows_handle_empty_invalid_and_exhausted_ranges() {
    for size in [0, 4, usize::MAX] {
        let mut values = [1, 2, 3];
        let mut windows = WindowsMut::new(&mut values, size);
        assert!(windows.next().is_none());
        assert!(windows.next().is_none());
    }
    let mut empty: [u8; 0] = [];
    assert!(WindowsMut::new(&mut empty, 1).next().is_none());

    let mut values = [1, 2, 3];
    let mut windows = WindowsMut::new(&mut values, 2);
    windows.next().unwrap()[1] = 20;
    assert_eq!(windows.next(), Some([20, 3].as_mut_slice()));
    assert!(windows.next().is_none());
    assert!(windows.next().is_none());
    assert_eq!(values, [1, 20, 3]);
}

#[test]
fn c17_gat_can_be_excluded_from_the_trait_object() {
    trait Store {
        type View<'a>
        where
            Self: Sized + 'a;

        fn view(&self) -> Self::View<'_>
        where
            Self: Sized;

        fn byte_len(&self) -> usize;
    }

    impl Store for String {
        type View<'a> = &'a str;

        fn view(&self) -> &str {
            self
        }

        fn byte_len(&self) -> usize {
            self.len()
        }
    }

    let text = String::from("Rust");
    assert_eq!(text.view(), "Rust");
    let store: &dyn Store = &text;
    assert_eq!(store.byte_len(), 4);
}
