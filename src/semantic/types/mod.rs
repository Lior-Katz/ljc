mod assignment;
mod contexts;
mod conversions;
mod numeric;

pub use contexts::NumericContext;
pub use conversions::{binary_numeric_promotion, unary_numeric_promotion};
pub use numeric::{IntegralMaybeBoxed, Numeric, NumericBoxed, NumericMaybeBoxed};

use crate::ast;
use crate::error::Diagnose;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    Primitive(Primitive),
    Boxed(Boxed),
    Null,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Primitive {
    Numeric(Numeric),
    Boolean,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Boxed {
    Numeric(NumericBoxed),
    Boolean,
}

pub enum BooleanMaybeBoxed {
    Primitive,
    Boxed,
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
            ast::Type::Boolean(_) => Ok(Primitive::Boolean.into()),
            ast::Type::Class(_) => Err(UnimplementedFeature::ReferenceTypes.at(*ty.span()).into()),
            ast::Type::Array(_) => Err(UnimplementedFeature::ArrayTypes.at(*ty.span()).into()),
        }
    }

    pub fn as_primitive(&self) -> Option<Primitive> {
        match self {
            Type::Primitive(primitive) => Some(primitive.clone()),
            Type::Boxed(_) | Type::Null => None,
        }
    }

    pub fn as_boxed(&self) -> Option<Boxed> {
        match self {
            Type::Boxed(boxed) => Some(boxed.clone()),
            Type::Primitive(_) | Type::Null => None,
        }
    }
}

impl Primitive {
    pub fn as_numeric(&self) -> Option<Numeric> {
        match self {
            Primitive::Numeric(numeric) => Some(numeric.clone()),
            Primitive::Boolean => None,
        }
    }
}

impl Boxed {
    pub fn as_numeric(&self) -> Option<NumericBoxed> {
        match self {
            Boxed::Numeric(numeric_boxed) => Some(numeric_boxed.clone()),
            Boxed::Boolean => None,
        }
    }
}

impl From<Primitive> for Type {
    fn from(value: Primitive) -> Self {
        Type::Primitive(value)
    }
}

impl From<Boxed> for Type {
    fn from(value: Boxed) -> Self {
        Type::Boxed(value)
    }
}

impl From<Numeric> for Primitive {
    fn from(value: Numeric) -> Self {
        Primitive::Numeric(value)
    }
}

impl From<NumericBoxed> for Boxed {
    fn from(value: NumericBoxed) -> Self {
        Boxed::Numeric(value)
    }
}

impl From<Numeric> for Type {
    fn from(value: Numeric) -> Self {
        Type::from(Primitive::from(value))
    }
}

impl From<NumericBoxed> for Type {
    fn from(value: NumericBoxed) -> Self {
        Type::from(Boxed::from(value))
    }
}

impl From<BooleanMaybeBoxed> for Type {
    fn from(value: BooleanMaybeBoxed) -> Self {
        match value {
            BooleanMaybeBoxed::Primitive => Primitive::Boolean.into(),
            BooleanMaybeBoxed::Boxed => Boxed::Boolean.into(),
        }
    }
}