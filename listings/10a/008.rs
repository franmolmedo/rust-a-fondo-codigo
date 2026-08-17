trait DomainError {
    fn attach<T>(&self, context: T);
}

fn main() {
    let _error: Option<Box<dyn DomainError>> = None;
    // error[E0038]: DomainError is not dyn compatible
}
