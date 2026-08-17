struct Validator<F> {
    predicate: F,
}

impl<F> Validator<F>
where
    F: Fn(&str) -> bool,
{
    fn is_valid(&self, input: &str) -> bool {
        (self.predicate)(input)
    }
}

fn main() {
    let minimum = 3;
    let validator = Validator {
        predicate: |input: &str| input.len() >= minimum,
    };
    assert!(validator.is_valid("rust"));
    assert!(!validator.is_valid("rs"));
}
