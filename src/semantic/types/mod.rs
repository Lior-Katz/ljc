mod assignment;
mod contexts;
mod conversions;
mod numeric;

pub use contexts::NumericContext;
pub use conversions::{binary_numeric_promotion, unary_numeric_promotion};
pub use numeric::{IntegralMaybeBoxed, Numeric, NumericBoxed, NumericMaybeBoxed};

use crate::ast;
use crate::ast::ClassType;
use crate::error::Diagnose;
use crate::semantic::error::{Error, NameResolutionKind, SemanticResult, UnimplementedFeature};
use crate::semantic::symbol_table::{Entity, ScopeId};
use crate::semantic::{SemanticAnalyzer, TypeId};

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    Primitive(Primitive),
    Boxed(Boxed),
    Null,
    Reference(TypeId),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BooleanMaybeBoxed {
    Primitive,
    Boxed,
}

impl SemanticAnalyzer<'_> {
    pub fn resolve(&self, ty: &ast::Type, scope: ScopeId) -> SemanticResult<Type> {
        match ty {
            ast::Type::Byte(_) => Ok(Numeric::Byte.into()),
            ast::Type::Short(_) => Ok(Numeric::Short.into()),
            ast::Type::Int(_) => Ok(Numeric::Int.into()),
            ast::Type::Long(_) => Ok(Numeric::Long.into()),
            ast::Type::Char(_) => Ok(Numeric::Char.into()),
            ast::Type::Float(_) => Ok(Numeric::Float.into()),
            ast::Type::Double(_) => Ok(Numeric::Double.into()),
            ast::Type::Boolean(_) => Ok(Primitive::Boolean.into()),
            ast::Type::Class(class_type) => self.resolve_class_type(class_type, scope),
            ast::Type::Array(_) => Err(UnimplementedFeature::ArrayTypes.at(*ty.span()).into()),
        }
    }

    fn resolve_class_type(&self, class_type: &ClassType, scope: ScopeId) -> SemanticResult<Type> {
        let ClassType { namespace, name: class } = class_type;
        if !namespace.is_empty() {
            Err(UnimplementedFeature::QualifiedType
                .at(*class_type.span())
                .into())
        } else {
            let name = class.identifier.identifier().value.clone();
            let Some(entity) = self.symbol_table.lookup(&name, scope) else {
                return Err(Error::UnknownSymbol(name).at(*class.span()).into());
            };
            match entity {
                Entity::Type(type_id) => Ok(Type::Reference(*type_id)),
                Entity::Method(_) => Err(Error::NameNotType(name, NameResolutionKind::Method)),
                Entity::Field(_) => Err(Error::NameNotType(name, NameResolutionKind::Field)),
                Entity::Variable(_) => {
                    Err(Error::NameNotType(name, NameResolutionKind::LocalVariable))
                }
            }
            .map_err(|e| e.at(*class.span()).into())
        }
    }
}

impl Type {
    pub fn as_primitive(&self) -> Option<Primitive> {
        match self {
            Type::Primitive(primitive) => Some(primitive.clone()),
            Type::Boxed(_) | Type::Null | Type::Reference(_) => None,
        }
    }

    pub fn as_boxed(&self) -> Option<Boxed> {
        match self {
            Type::Boxed(boxed) => Some(boxed.clone()),
            Type::Primitive(_) | Type::Null | Type::Reference(_) => None,
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
