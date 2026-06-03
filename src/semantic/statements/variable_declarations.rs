use crate::ast::{Expression, Modifiers, VariableDeclaration, VariableInitializer};
use crate::error::Diagnose;
use crate::file::Span;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceIter, SemanticResult, TypeMismatch, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::types::Type;

impl SemanticAnalyzer<'_> {
    #[allow(unused_variables)]
    pub fn variable_declaration(
        &mut self,
        var_decl: &VariableDeclaration,
        modifiers: &Modifiers,
        span: Span,
    ) -> SemanticResult {
        let declaration_type = Type::resolve(&var_decl.variable_type)?;
        (&var_decl.declarators).coalesce(|declarator| match &declarator.initializer {
            Some(VariableInitializer::Expression(e)) => {
                self.check_initializer(&declaration_type, e, *e.span())?;
                Ok(())
            }
            Some(VariableInitializer::ArrayInitializer(a)) => {
                Err(UnimplementedFeature::ArrayInitializer.at(a.span).into())
            }
            None => Ok(()),
        })
    }
    fn check_initializer(
        &self,
        expected_type: &Type,
        expression: &Expression,
        span: Span,
    ) -> SemanticResult {
        let expression_result = self.expression(expression)?;
        match expression_result {
            ExpressionResult::Void => Err(TypeMismatch::VoidExpression.at(span).into()),
            ExpressionResult::Value(ref ty) | ExpressionResult::Variable(ref ty) => {
                if expected_type.is_assignable_from(ty) {
                    Ok(())
                } else {
                    Err(TypeMismatch::IncompatibleAssignment.at(span).into())
                }
            }
        }
    }
}
