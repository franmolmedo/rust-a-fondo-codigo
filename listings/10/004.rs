#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
    Overflow,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, DivisionError> {
    if divisor == 0 {
        Err(DivisionError::DivisionByZero)
    } else {
        dividend.checked_div(divisor).ok_or(DivisionError::Overflow)
    }
}

fn main() {
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err(DivisionError::DivisionByZero));
    assert_eq!(divide(i32::MIN, -1), Err(DivisionError::Overflow));
    assert_eq!(divide(i32::MIN, 1), Ok(i32::MIN));
}
