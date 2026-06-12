use crate::ast::identifiers::{Identifier, IdentifierKind, TypeIdentifier};
use crate::collections::{AtLeastOne, Multiple};
use crate::file::Span;

pub type TypeList = AtLeastOne<Type>;
pub type TypeName = AtLeastOne<Identifier>;
pub type ClassTypePartList = Multiple<ClassTypePart>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ClassType {
    pub namespace: ClassTypePartList,
    pub name: ClassTypePart<TypeIdentifier>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Type {
    // primitive types
    Byte(Span),
    Short(Span),
    Int(Span),
    Long(Span),
    Char(Span),
    Float(Span),
    Double(Span),
    Boolean(Span),

    // reference types
    Class(ClassType),
    Array(ArrayType),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum TypeOrVoid {
    Type(Type),
    Void(Span),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ClassTypePart<T: IdentifierKind = Identifier> {
    pub identifier: T,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ArrayType {
    pub element_type: Box<Type>,
}

impl TypeOrVoid {
    pub fn span(&self) -> &Span {
        match self {
            TypeOrVoid::Type(ty) => ty.span(),
            TypeOrVoid::Void(span) => span,
        }
    }
}

impl Type {
    pub fn span(&self) -> &Span {
        match self {
            Self::Byte(span)
            | Self::Short(span)
            | Self::Int(span)
            | Self::Long(span)
            | Self::Char(span)
            | Self::Float(span)
            | Self::Double(span)
            | Self::Boolean(span) => span,

            Self::Class(class_type) => class_type.span(),
            Self::Array(array_type) => array_type.span(),
        }
    }
}

impl ClassType {
    pub fn span(&self) -> &Span {
        match  self.namespace.get(0) {
            Some(part) => part.span(),
            None => &self.name.span()
        }
    }
}

impl ArrayType {
    pub fn span(&self) -> &Span {
        self.element_type.span()
    }
}

impl ClassTypePart {
    pub fn span(&self) -> &Span {
        &self.identifier.span
    }
}

impl ClassTypePart<TypeIdentifier> {
    pub fn span(&self) -> &Span {
        self.identifier.span()
    }
}