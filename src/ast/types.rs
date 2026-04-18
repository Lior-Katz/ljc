use crate::ast::identifiers::Identifier;

pub type ClassType = Vec<ClassTypePart>;

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
