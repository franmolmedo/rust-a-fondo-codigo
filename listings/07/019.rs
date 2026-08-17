#[derive(Debug, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Default)]
struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    workers: Option<usize>,
}

#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingHost,
    MissingPort,
    ZeroWorkers,
}

impl ServerConfigBuilder {
    fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    fn port(mut self, value: u16) -> Self {
        self.port = Some(value);
        self
    }

    fn workers(mut self, value: usize) -> Self {
        self.workers = Some(value);
        self
    }

    fn build(self) -> Result<ServerConfig, ConfigError> {
        let host = self.host.ok_or(ConfigError::MissingHost)?;
        let port = self.port.ok_or(ConfigError::MissingPort)?;
        let workers = self.workers.unwrap_or(4);
        if workers == 0 {
            return Err(ConfigError::ZeroWorkers);
        }
        Ok(ServerConfig {
            host,
            port,
            workers,
        })
    }
}

fn main() {
    let config = ServerConfigBuilder::default()
        .host("127.0.0.1")
        .port(8080)
        .workers(8)
        .build()
        .unwrap();

    assert_eq!(config.workers, 8);
    assert_eq!(
        ServerConfigBuilder::default().port(8080).build(),
        Err(ConfigError::MissingHost)
    );
}
