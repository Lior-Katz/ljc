use crate::ast;
use crate::ast::{ClassBodyDeclaration, Modified, Modifiers};
use crate::error::Diagnose;
use crate::semantic::classes::members::class_member;
use crate::semantic::error::{Coalesce, SemanticResult, UnimplementedFeature};

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
            ClassBodyDeclaration::InstanceInitializer(_) => {
                Err(UnimplementedFeature::InstanceInitializer
                    .at(body_declaration.span().clone())
                    .into())
            }
            ClassBodyDeclaration::StaticInitializer(_) => {
                Err(UnimplementedFeature::StaticInitializer
                    .at(body_declaration.span().clone())
                    .into())
            }
        })
}
