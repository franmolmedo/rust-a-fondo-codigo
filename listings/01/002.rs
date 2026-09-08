struct ConnectionGuard(&'static str);

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        println!("cerrando {}", self.0);
    }
}

fn main() {
    let _database = ConnectionGuard("base de datos");
    {
        let _cache = ConnectionGuard("cache");
        println!("trabajando con ambas");
    } // aquí se destruye `_cache` y se imprime su mensaje de cierre
    println!("solo queda la base de datos");
} // al final se destruye `_database` y se imprime su mensaje de cierre
