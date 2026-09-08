// Aunque Config sea propio, quien llama también depende del error de toml.
pub fn parse(input: &str) -> Result<Config, toml::de::Error>
