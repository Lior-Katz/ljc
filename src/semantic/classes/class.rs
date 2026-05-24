use crate::ast;
use crate::ast::{ClassBodyDeclaration, Modified, Modifiers};
use crate::semantic::classes::members::class_member;
use crate::semantic::error::{Coalesce, SemanticResult};

#[allow(unused_variables)]
pub fn class_declaration(
    class_decl: &ast::NormalClassDeclaration,
    modifiers: &Modifiers,
) -> SemanticResult {
    class_decl
        .body
        .coalesce(|body_declaration| match body_declaration {
            ClassBodyDeclaration::ClassMember(Modified { modifiers, item }) => {
                class_member(item, modifiers)
            }
            _ => todo!("Only class member declarations are supported for now"),
        })
}
