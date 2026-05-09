pub type Identifier = String;

#[derive(Debug)]
pub struct TypeIdentifier(Identifier);

impl TypeIdentifier {
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }

    pub fn from(identifier: Identifier) -> Self {
        Self(identifier)
    }
}
