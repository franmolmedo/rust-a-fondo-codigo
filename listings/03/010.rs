fn add_stock(current: u32, incoming: u32) -> Option<u32> {
    current.checked_add(incoming)
}
