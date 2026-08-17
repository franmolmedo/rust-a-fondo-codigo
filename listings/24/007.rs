use std::marker::PhantomPinned;

struct AddressSensitive {
    data: String,
    _pin: PhantomPinned,
}
