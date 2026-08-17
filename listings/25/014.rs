// Aunque `Config` sea tuyo, esta firma promete `toml` para siempre:
pub fn parse(input: &str) -> Result<Config, toml::de::Error>
