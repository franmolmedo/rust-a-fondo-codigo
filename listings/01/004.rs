fn main() {
    // El compilador usa la representación imposible (puntero nulo)
    // para codificar None: la seguridad no añade ni un byte.
    assert_eq!(size_of::<Option<&u8>>(), size_of::<&u8>());
    assert_eq!(size_of::<Option<Box<u64>>>(), size_of::<Box<u64>>());
    println!("Option sobre punteros: mismo tamaño que el puntero desnudo");
}
