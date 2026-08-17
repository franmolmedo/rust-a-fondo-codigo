struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {}
}

fn main() {
    let guard = Guard;
    guard.drop();
    // error[E0040]: explicit use of destructor method
}
