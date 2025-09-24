#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value(pub Vec<u8>);

impl Key {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Value {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
