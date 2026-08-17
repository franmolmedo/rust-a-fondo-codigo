// Dos locks: la invariante "reserved <= total" puede observarse rota.
struct Inventory {
    total: Mutex<u32>,
    reserved: Mutex<u32>,
}

// Un lock: la invariante cambia atómicamente o no cambia.
struct InventoryAtomic {
    counts: Mutex<Counts>,
}

struct Counts {
    total: u32,
    reserved: u32,
}
