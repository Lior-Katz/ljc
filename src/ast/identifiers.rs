pub trait IdentifierKind {}
pub type Identifier = String;

#[derive(Debug)]
pub struct TypeIdentifier(Identifier);
impl IdentifierKind for TypeIdentifier {}
impl IdentifierKind for Identifier {}

pub struct InvalidTypeIdentifier;

impl TypeIdentifier {
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }

    pub fn from(identifier: Identifier) -> Result<Self, InvalidTypeIdentifier> {
        let type_identifier_exclude = ["permits", "record", "sealed", "var", "yield"];
        if type_identifier_exclude.contains(&identifier.as_str()) {
            Err(InvalidTypeIdentifier)
        } else {
            Ok(Self(identifier))
        }
    }
}
