#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

#[derive(Debug, PartialEq, Eq)]
enum OrderError {
    CannotPayFrom(OrderStatus),
}

struct Order {
    status: OrderStatus,
}

impl Order {
    fn pay(&mut self) -> Result<(), OrderError> {
        match self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Paid;
                Ok(())
            }
            status => Err(OrderError::CannotPayFrom(status)),
        }
    }
}

fn main() {
    let mut order = Order {
        status: OrderStatus::Pending,
    };
    assert_eq!(order.pay(), Ok(()));
    assert_eq!(order.pay(), Err(OrderError::CannotPayFrom(OrderStatus::Paid)));
}
