mod class;
use crate::ast::{
    ClassDeclaration, InterfaceDeclaration, Modifiers, TopLevelClassOrInterfaceDeclaration,
};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};

mod members;

impl SemanticAnalyzer {
    pub(super) fn top_level_class_or_interface_declaration(
        &mut self,
        declaration: &TopLevelClassOrInterfaceDeclaration,
        modifiers: &Modifiers,
    ) -> SemanticResult {
        let span = declaration.span().clone();
        match declaration {
            TopLevelClassOrInterfaceDeclaration::Class(ClassDeclaration::NormalClass(c)) => {
                self.class_declaration(c, modifiers)
            }
            TopLevelClassOrInterfaceDeclaration::Class(ClassDeclaration::Record(_)) => {
                Err(UnimplementedFeature::RecordClass.at(span).into())
            }
            TopLevelClassOrInterfaceDeclaration::Class(ClassDeclaration::Enum(_)) => {
                Err(UnimplementedFeature::EnumClass.at(span).into())
            }
            TopLevelClassOrInterfaceDeclaration::Interface(
                InterfaceDeclaration::NormalInterface(_),
            ) => Err(UnimplementedFeature::Interface.at(span).into()),
            TopLevelClassOrInterfaceDeclaration::Interface(
                InterfaceDeclaration::AnnotationInterface(_),
            ) => Err(UnimplementedFeature::AnnotationInterface.at(span).into()),
        }
    }
}
