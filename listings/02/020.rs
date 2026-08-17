fn checked_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    a.checked_div(b)
}
