fn require_static<T: 'static>(_value: T) {}

fn main() {
    let owned = String::from("propio");
    require_static(owned);
}
