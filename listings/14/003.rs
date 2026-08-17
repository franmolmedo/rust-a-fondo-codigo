struct UserId(u64);
struct OrderId(u64);

fn cancel_order(_id: OrderId) {}

fn main() {
    let user = UserId(9);
    cancel_order(user);
    // error[E0308]: se esperaba OrderId, se encontró UserId
}
