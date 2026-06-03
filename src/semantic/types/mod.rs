mod assignment;
mod numeric;

use crate::ast;
use crate::error::Diagnose;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};
pub use numeric::Numeric;

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    Numeric(Numeric),
    Boolean,
    Null,
}

impl Type {
    pub fn resolve(ty: &ast::Type) -> SemanticResult<Self> {
        match ty {
            ast::Type::Byte(_) => Ok(Numeric::Byte.into()),
            ast::Type::Short(_) => Ok(Numeric::Short.into()),
            ast::Type::Int(_) => Ok(Numeric::Int.into()),
            ast::Type::Long(_) => Ok(Numeric::Long.into()),
            ast::Type::Char(_) => Ok(Numeric::Char.into()),
            ast::Type::Float(_) => Ok(Numeric::Float.into()),
            ast::Type::Double(_) => Ok(Numeric::Double.into()),
            ast::Type::Boolean(_) => Ok(Self::Boolean),
            ast::Type::Class(_) => Err(UnimplementedFeature::ReferenceTypes.at(*ty.span()).into()),
            ast::Type::Array(_) => Err(UnimplementedFeature::ArrayTypes.at(*ty.span()).into()),
        }
    }
}

impl From<Numeric> for Type {
    fn from(value: Numeric) -> Self {
        Self::Numeric(value)
    }
}
