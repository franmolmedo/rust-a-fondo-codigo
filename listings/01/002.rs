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
    } // la cache se cierra exactamente aquí
    println!("solo queda la base de datos");
} // la base de datos se cierra al final
