enum List<T> {
    Nil,
    Cons(T, List<T>),
    // error[E0072]: recursive type `List` has infinite size
}
