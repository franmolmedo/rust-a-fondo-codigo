impl Document {
    pub fn title(&self) -> &str;
    pub fn rename(&mut self, title: Title);
    pub fn into_bytes(self) -> Vec<u8>;
}
