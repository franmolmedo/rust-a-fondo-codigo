let big: Box<[u8; 4096]> = Box::new([0; 4096]);
assert_eq!(std::mem::size_of_val(&big), std::mem::size_of::<usize>());
