#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Shipped,
    Cancelled,
}

#[derive(Debug, PartialEq)]
enum CancelError {
    AlreadyShipped,
    AlreadyCancelled,
}

fn cancel(status: &mut Status) -> Result<(), CancelError> {
    match status {
        Status::Pending => {
            *status = Status::Cancelled;
            Ok(())
        }
        Status::Shipped => Err(CancelError::AlreadyShipped),
        Status::Cancelled => Err(CancelError::AlreadyCancelled),
    }
}

fn main() {
    let mut status = Status::Pending;
    assert_eq!(cancel(&mut status), Ok(()));
    assert_eq!(cancel(&mut status), Err(CancelError::AlreadyCancelled));
}
