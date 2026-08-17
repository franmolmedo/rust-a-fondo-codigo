let settings = Settings {
    raw_port: String::from(" 9090 "),
    parsed: OnceCell::new(),
};

assert_eq!(settings.port(), 9090); // parsea la primera vez
assert_eq!(settings.port(), 9090); // reutiliza sin parsear
