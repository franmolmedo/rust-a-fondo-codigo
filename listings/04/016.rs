struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("guard liberado");
    }
}

fn acquire_lock() -> Guard {
    Guard
}

fn do_protected_work() {}
fn do_unlocked_work() {}

fn main() {
    let guard = acquire_lock();
    do_protected_work();
    drop(guard);
    do_unlocked_work();
}
