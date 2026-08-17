enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

fn sum(list: &List<i32>) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(value, rest) => value + sum(rest),
    }
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
assert_eq!(sum(&list), 3);
