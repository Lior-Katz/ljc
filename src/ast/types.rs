use crate::ast::identifiers::Identifier;
use crate::collections::AtLeastOne;

pub type ClassType = AtLeastOne<ClassTypePart>;
pub type ClassTypeList = AtLeastOne<ClassType>;
pub type TypeName = AtLeastOne<Identifier>;

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
pub struct ClassTypePart {
    pub identifier: Identifier,
}

#[derive(Debug)]
pub struct ArrayType {
    pub element_type: Box<Type>,
}
