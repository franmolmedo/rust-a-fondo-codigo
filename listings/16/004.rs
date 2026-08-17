fn max_ref<'a, T: Ord>(left: &'a T, right: &'a T) -> &'a T {
    if left >= right { left } else { right }
}

fn max_owned<T>(left: &T, right: &T) -> T
where
    T: Ord + Clone,
{
    max_ref(left, right).clone()
}

fn main() {
    let left = String::from("Ada");
    let right = String::from("Grace");
    assert_eq!(max_ref(&left, &right), "Grace");
    assert_eq!(max_owned(&left, &right), "Grace");
}
