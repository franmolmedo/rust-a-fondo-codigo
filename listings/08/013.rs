#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, DivisionError> {
    if divisor == 0 {
        Err(DivisionError::DivisionByZero)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err(DivisionError::DivisionByZero));
}
