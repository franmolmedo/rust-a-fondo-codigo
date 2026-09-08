fn remaining(capacity: u32, occupied: u32) -> Option<u32> {
    capacity.checked_sub(occupied)
}

fn main() {
    let capacity = 12;
    let occupied = 5;
    match remaining(capacity, occupied) {
        Some(free) => println!("Free spaces: {free}"),
        None => eprintln!("Occupied spaces exceed capacity"),
    }
}

#[cfg(test)]
mod tests {
    use super::remaining;

    #[test]
    fn computes_free_spaces() {
        assert_eq!(remaining(12, 5), Some(7));
        assert_eq!(remaining(12, 12), Some(0));
    }

    #[test]
    fn rejects_over_capacity() {
        assert_eq!(remaining(12, 13), None);
    }
}
