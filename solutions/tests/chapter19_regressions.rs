use course_solutions::abstraction::c19::{
    Increase, Negate, Operation, OperationKind, Scale, run_closed, run_dynamic,
};

#[test]
fn c19_enum_and_dyn_operations_agree_at_integer_boundaries() {
    for operand in [i64::MIN, -1, 0, 1, i64::MAX] {
        let closed = [
            OperationKind::Increase(operand),
            OperationKind::Scale(operand),
            OperationKind::Negate,
        ];
        let dynamic: Vec<Box<dyn Operation>> = vec![
            Box::new(Increase(operand)),
            Box::new(Scale(operand)),
            Box::new(Negate),
        ];
        for value in [i64::MIN, -1, 0, 1, i64::MAX] {
            let expected = [
                i128::from(value) + i128::from(operand),
                i128::from(value) * i128::from(operand),
                -i128::from(value),
            ];
            assert_eq!(run_closed(&closed, value), expected);
            assert_eq!(run_dynamic(&dynamic, value), expected);
        }
    }
}

#[test]
fn c19_trait_upcasting_preserves_the_supertrait_operation() {
    trait Parent {
        fn label(&self) -> &str;
    }
    trait Child: Parent {
        fn byte_len(&self) -> usize;
    }
    impl Parent for String {
        fn label(&self) -> &str {
            self
        }
    }
    impl Child for String {
        fn byte_len(&self) -> usize {
            self.len()
        }
    }
    let text = String::from("Rust");
    let child: &dyn Child = &text;
    assert_eq!(child.byte_len(), 4);
    let parent: &dyn Parent = child;
    assert_eq!(parent.label(), "Rust");
}
