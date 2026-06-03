use crate::ast;
use crate::ast::{ClassBodyDeclaration, Modified, Modifiers};
use crate::error::Diagnose;
use crate::semantic::error::{CoalesceIter, SemanticResult, UnimplementedFeature};
use crate::semantic::SemanticAnalyzer;

impl SemanticAnalyzer<'_> {
    #[allow(unused_variables)]
    pub(super) fn class_declaration(
        &mut self,
        class_decl: &ast::ClassDeclaration,
        modifiers: &Modifiers,
    ) -> SemanticResult {
        class_decl
            .body
            .coalesce(|body_declaration| match body_declaration {
                ClassBodyDeclaration::ClassMember(Modified { modifiers, item }) => {
                    self.class_member(item, modifiers)
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
}
