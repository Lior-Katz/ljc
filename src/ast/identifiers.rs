use crate::parser::ParseError;

pub type Identifier = String;

#[derive(Debug)]
pub struct TypeIdentifier(Identifier);

impl TypeIdentifier {
    pub fn identifier(&self) -> &Identifier {
        &self.0
    }
}

impl TryFrom<Identifier> for TypeIdentifier {
    type Error = ParseError;
    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        let type_identifier_exclude = ["permits", "record", "sealed", "var", "yield"];
        if type_identifier_exclude.contains(&value.as_str()) {
            Err(ParseError::NoProduction)
        } else {
            Ok(TypeIdentifier(value))
        }
    }
}

