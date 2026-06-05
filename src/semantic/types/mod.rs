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
use crate::semantic::types::numeric::{Integral, IntegralBoxed};

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    Numeric(Numeric),
    NumericBoxed(NumericBoxed),
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

impl From<NumericBoxed> for Type {
    fn from(value: NumericBoxed) -> Self {
        Self::NumericBoxed(value)
    }
}

impl From<NumericMaybeBoxed> for Type {
    fn from(value: NumericMaybeBoxed) -> Self {
        match value {
            NumericMaybeBoxed::Primitive(numeric) => numeric.into(),
            NumericMaybeBoxed::Boxed(numeric_boxed) => numeric_boxed.into(),
        }
    }
}

impl From<Integral> for Type {
    fn from(value: Integral) -> Self {
        Self::from(Numeric::from(value))
    }
}

impl From<IntegralBoxed> for Type {
    fn from(value: IntegralBoxed) -> Self {
        Self::from(NumericBoxed::from(value))
    }
}

impl From<IntegralMaybeBoxed> for Type {
    fn from(value: IntegralMaybeBoxed) -> Self {
        match value {
            IntegralMaybeBoxed::Primitive(integral) => integral.into(),
            IntegralMaybeBoxed::Boxed(integral_boxed) => integral_boxed.into(),
        }
    }
}
