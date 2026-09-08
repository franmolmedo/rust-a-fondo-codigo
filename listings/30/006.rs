use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();

tx.send(1).unwrap();
tx2.send(2).unwrap();
drop(tx);
drop(tx2);

assert_eq!(rx.into_iter().collect::<Vec<_>>(), vec![1, 2]);
