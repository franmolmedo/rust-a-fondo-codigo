// Dos mutex: hay que coordinar ambos accesos para mantener reserved <= total.
struct Inventory {
    total: Mutex<u32>,
    reserved: Mutex<u32>,
}

// Un mutex: ninguna otra guarda puede observar una actualización a medias.
struct InventoryAtomic {
    counts: Mutex<Counts>,
}

struct Counts {
    total: u32,
    reserved: u32,
}
