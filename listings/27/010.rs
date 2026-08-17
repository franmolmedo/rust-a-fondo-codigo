#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "json")]
impl Order {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Order siempre serializa")
    }
}
