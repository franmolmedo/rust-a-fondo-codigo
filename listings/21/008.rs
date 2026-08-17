use std::rc::Rc;

let mut current = Rc::new(vec![1, 2]);
assert!(Rc::get_mut(&mut current).is_some());

let snapshot = Rc::clone(&current);
assert!(Rc::get_mut(&mut current).is_none());

Rc::make_mut(&mut current).push(3); // clona Vec porque existe snapshot
assert_eq!(&*snapshot, &[1, 2]);
assert_eq!(&*current, &[1, 2, 3]);
