use crate::ast::{ClassBodyDeclaration, ClassDeclaration, Modified, Modifiers};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceIter, SemanticResult, UnimplementedFeature};

impl<'a> SemanticAnalyzer<'a> {
    #[allow(unused_variables)]
    pub(super) fn class_declaration(
        &mut self,
        class_decl: &'a ClassDeclaration,
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
