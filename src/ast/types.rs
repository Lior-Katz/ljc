use crate::ast::identifiers::{Identifier, IdentifierKind, TypeIdentifier};
use crate::collections::{AtLeastOne, Multiple};

pub type ClassTypeList = AtLeastOne<ClassType>;
pub type TypeName = AtLeastOne<Identifier>;
pub type ClassTypePartList = Multiple<ClassTypePart>;

#[derive(Debug)]
pub struct ClassType {
    pub namespace: ClassTypePartList,
    pub name: ClassTypePart<TypeIdentifier>,
}

#[derive(Debug)]
pub enum Type {
    // primitive types
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Boolean,
    Void,

    // reference types
    Class(ClassType),
    Array(ArrayType),
}

#[derive(Debug)]
pub struct ClassTypePart<T: IdentifierKind = Identifier> {
    pub identifier: T,
}

#[derive(Debug)]
pub struct ArrayType {
    pub element_type: Box<Type>,
}
