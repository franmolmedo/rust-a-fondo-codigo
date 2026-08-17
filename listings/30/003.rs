let names = vec![String::from("Ada"), String::from("Grace")];

let handle = thread::spawn(move || names.len());
assert_eq!(handle.join().unwrap(), 2);
