use course_solutions::fundamentals::c10::{
    OrderError, OrderStatus, Quantity, QuantityError, RepositoryError, User, cancel,
    repository_find,
};

#[test]
fn c10_quantity_distinguishes_the_configured_limit_from_integer_overflow() {
    let four = Quantity::new(4, 10).unwrap();
    let six = Quantity::new(6, 10).unwrap();
    assert_eq!(four.checked_add(six, 10).map(Quantity::get), Ok(10));
    assert_eq!(
        four.checked_add(six, 9),
        Err(QuantityError::AboveMaximum {
            maximum: 9,
            actual: 10,
        })
    );

    let largest = Quantity::new(u32::MAX, u32::MAX).unwrap();
    let one = Quantity::new(1, u32::MAX).unwrap();
    assert_eq!(
        largest.checked_add(one, u32::MAX),
        Err(QuantityError::Overflow)
    );
}

#[test]
fn c10_repository_returns_three_distinct_results_and_an_owned_user() {
    let found = {
        let users = [User { id: 7 }];
        assert_eq!(repository_find(&users, 8, true), Ok(None));
        assert_eq!(
            repository_find(&users, 7, false),
            Err(RepositoryError::Unavailable)
        );
        repository_find(&users, 7, true)
    };
    assert_eq!(found, Ok(Some(User { id: 7 })));
}

#[test]
fn c10_rejected_cancellations_preserve_the_order_status() {
    for (mut status, expected) in [
        (OrderStatus::Shipped, OrderError::AlreadyShipped),
        (OrderStatus::Cancelled, OrderError::AlreadyCancelled),
    ] {
        let before = status;
        assert_eq!(cancel(&mut status), Err(expected));
        assert_eq!(status, before);
    }
}
