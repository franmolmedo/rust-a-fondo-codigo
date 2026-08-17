const fn port_from_literal(value: u16) -> u16 {
    value
}

macro_rules! checked_port {
    ($value:literal) => { port_from_literal($value) };
    ($($other:tt)*) => {
        compile_error!("checked_port! espera un único literal entero entre 0 y 65535")
    };
}

fn main() {
    let base = 8000;
    let _ = checked_port!(base + 80);
}
