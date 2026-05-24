mod method;

use crate::ast::{ClassMemberDeclaration, Modifiers};
use crate::semantic::error::SemanticResult;
use method::method;

pub fn class_member(member: &ClassMemberDeclaration, modifiers: &Modifiers) -> SemanticResult {
    match member {
        ClassMemberDeclaration::Method(m) => method(m, modifiers),
        _ => todo!("Only method declarations are supported for now"),
    }
}
