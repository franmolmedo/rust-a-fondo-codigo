fn drain(mut stack: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    while let Some(value) = stack.pop() {
        output.push(value);
    }
    output
}

fn positive(value: Option<i32>) -> Result<i32, &'static str> {
    let Some(value) = value else {
        return Err("ausente");
    };

    if let 1..=i32::MAX = value {
        Ok(value)
    } else {
        Err("no positivo")
    }
}

fn main() {
    assert_eq!(drain(vec![1, 2, 3]), [3, 2, 1]);
    assert_eq!(positive(Some(4)), Ok(4));
    assert!(matches!(positive(Some(0)), Err(_)));
}
