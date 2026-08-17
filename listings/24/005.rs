use std::marker::PhantomPinned;

struct Stable {
    name: String,
    _pin: PhantomPinned,
}

let pinned = Box::pin(Stable {
    name: String::from("rust"),
    _pin: PhantomPinned,
});
let before = pinned.as_ref().get_ref() as *const Stable;

let moved_handle = pinned; // se mueve Pin<Box<_>>, no Stable
let after = moved_handle.as_ref().get_ref() as *const Stable;

assert_eq!(before, after);
assert_eq!(moved_handle.as_ref().get_ref().name.as_str(), "rust");
