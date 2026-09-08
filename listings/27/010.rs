#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Order {
    pub id: u64,
}

#[cfg(feature = "json")]
impl Order {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
