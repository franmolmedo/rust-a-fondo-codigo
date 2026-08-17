#[derive(Debug, PartialEq)]
struct PaidOrder {
    id: u64,
}

#[derive(Debug, PartialEq)]
struct ShippedOrder {
    id: u64,
}

impl PaidOrder {
    fn ship(self) -> ShippedOrder {
        ShippedOrder { id: self.id }
    }
}

fn main() {
    let paid = PaidOrder { id: 7 };
    let shipped = paid.ship();
    assert_eq!(shipped, ShippedOrder { id: 7 });
}
