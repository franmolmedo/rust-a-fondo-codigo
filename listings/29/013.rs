impl Snapshot {
    pub fn as_bytes(&self) -> &[u8] { /* vista, O(1), sin asignar */ }
    pub fn to_vec(&self) -> Vec<u8> { /* copia: asigna y duplica */ }
    pub fn into_bytes(self) -> Vec<u8> { /* consume: entrega el buffer sin copiar */ }
    pub fn is_empty(&self) -> bool { /* predicado barato */ }
    pub fn try_merge(&self, other: &Snapshot) -> Result<Snapshot, MergeError> { /* fallible */ }
}
