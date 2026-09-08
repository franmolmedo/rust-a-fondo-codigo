enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

fn sum(list: &List<i32>) -> Option<i32> {
    match list {
        List::Nil => Some(0),
        List::Cons(value, rest) => value.checked_add(sum(rest)?),
    }
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
assert_eq!(sum(&list), Some(3));
let overflowing = List::Cons(i32::MAX, Box::new(List::Cons(1, Box::new(List::Nil))));
assert_eq!(sum(&overflowing), None);
