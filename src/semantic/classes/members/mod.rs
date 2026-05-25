mod method;

use crate::ast::{ClassMemberDeclaration, Modifiers};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};

impl SemanticAnalyzer {
    pub(super) fn class_member(&mut self, member: &ClassMemberDeclaration, modifiers: &Modifiers) -> SemanticResult {
        let span = member.span().clone();
        match member {
            ClassMemberDeclaration::Method(m) => self.method(m, modifiers),
            ClassMemberDeclaration::NestedClass(_) => {
                Err(UnimplementedFeature::NestedClass.at(span).into())
            }
            ClassMemberDeclaration::NestedInterface(_) => {
                Err(UnimplementedFeature::NestedInterface.at(span).into())
            }
            ClassMemberDeclaration::Field { .. } => {
                Err(UnimplementedFeature::ClassField.at(span).into())
            }
            ClassMemberDeclaration::Constructor { .. } => {
                Err(UnimplementedFeature::Constructor.at(span).into())
            }
            ClassMemberDeclaration::CompactConstructor { .. } => {
                Err(UnimplementedFeature::CompactConstructor.at(span).into())
            }
        }
    }
}
