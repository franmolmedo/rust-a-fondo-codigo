#[derive(Debug, PartialEq)]
struct Config {
    debug: bool,
    port: u16,
}

struct ConfigBuilder {
    debug: bool,
    port: u16,
}

impl ConfigBuilder {
    fn new() -> Self {
        Self {
            debug: false,
            port: 8080,
        }
    }

    fn debug(mut self, value: bool) -> Self {
        self.debug = value;
        self
    }

    fn port(mut self, value: u16) -> Self {
        self.port = value;
        self
    }

    fn build(self) -> Config {
        Config {
            debug: self.debug,
            port: self.port,
        }
    }
}

fn main() {
    let config = ConfigBuilder::new()
        .debug(true)
        .port(3000)
        .build();

    assert_eq!(
        config,
        Config {
            debug: true,
            port: 3000,
        }
    );
}
