use std::thread;

let names = vec![String::from("Ada"), String::from("Grace")];

let handle = thread::spawn(|| names.len());
// error[E0373]: closure may outlive the current function,
// but it borrows `names`, which is owned by the current function
