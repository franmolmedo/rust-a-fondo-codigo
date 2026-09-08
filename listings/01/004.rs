fn main() {
    // Rust representa None mediante un puntero nulo.
    // En estos dos tipos, Option no necesita espacio adicional.
    assert_eq!(size_of::<Option<&u8>>(), size_of::<&u8>());
    assert_eq!(size_of::<Option<Box<u64>>>(), size_of::<Box<u64>>());
    println!("En estos dos casos, Option ocupa lo mismo que el tipo que contiene");
}
