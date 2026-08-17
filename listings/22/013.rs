use std::cell::OnceCell;

struct Settings {
    raw_port: String,
    parsed: OnceCell<u16>,
}

impl Settings {
    fn port(&self) -> u16 {
        *self.parsed.get_or_init(|| self.raw_port.trim().parse().unwrap_or(8080))
    }
}
