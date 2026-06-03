mod class;

use crate::ast::{Modifiers, TypeDeclaration};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};

mod members;

impl SemanticAnalyzer {
    pub(super) fn top_level_class_or_interface_declaration(
        &mut self,
        declaration: &TypeDeclaration,
        modifiers: &Modifiers,
    ) -> SemanticResult {
        let span = declaration.span().clone();
        match declaration {
            TypeDeclaration::Class(c) => self.class_declaration(c, modifiers),
            TypeDeclaration::Record(_) => Err(UnimplementedFeature::RecordClass.at(span).into()),
            TypeDeclaration::Enum(_) => Err(UnimplementedFeature::EnumClass.at(span).into()),
            TypeDeclaration::Interface(_) => Err(UnimplementedFeature::Interface.at(span).into()),
            TypeDeclaration::AnnotationInterface(_) => {
                Err(UnimplementedFeature::AnnotationInterface.at(span).into())
            }
        }
    }
}
