use crate::file::Span;

pub trait IdentifierKind {}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Identifier {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct TypeIdentifier(Identifier);
impl IdentifierKind for TypeIdentifier {}
impl IdentifierKind for Identifier {}

pub struct InvalidTypeIdentifier;

impl TypeIdentifier {
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }

    pub fn span(&self) -> &Span {
        &self.0.span
    }

    pub fn from(identifier: Identifier) -> Result<Self, InvalidTypeIdentifier> {
        let type_identifier_exclude = ["permits", "record", "sealed", "var", "yield"];
        if type_identifier_exclude.contains(&identifier.value.as_str()) {
            Err(InvalidTypeIdentifier)
        } else {
            Ok(Self(identifier))
        }
    }
}
