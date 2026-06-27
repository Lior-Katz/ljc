use crate::ast::{
    Expression, Modifiers, VariableDeclaration, VariableDeclaratorId, VariableInitializer,
};
use crate::error::Diagnose;
use crate::file::Span;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::ast_tags::VariableDeclaratorAttributes;
use crate::semantic::error::{CoalesceIter, SemanticResult, TypeMismatch, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::{Entity, ScopeId};
use crate::semantic::types::Type;

impl<'a> SemanticAnalyzer<'a> {
    #[expect(unused_variables)]
    pub fn variable_declaration(
        &mut self,
        var_decl: &'a VariableDeclaration,
        modifiers: &Modifiers,
        span: Span,
        scope: ScopeId,
    ) -> SemanticResult {
        let declaration_type = self.resolve(&var_decl.variable_type, scope)?;
        (&var_decl.declarators).coalesce(|declarator| {
            if let VariableDeclaratorId::Named(name) = &declarator.name {
                self.symbol_table
                    .scope_mut(scope)
                    .put(name.value.clone(), Entity::Variable(declarator));
            }
            self.attributes
                .insert(declarator, VariableDeclaratorAttributes { ty: declaration_type.clone() });
            match &declarator.initializer {
                Some(VariableInitializer::Expression(e)) => {
                    self.check_initializer(&declaration_type, e, *e.span(), scope)?;
                    Ok(())
                }
                Some(VariableInitializer::ArrayInitializer(a)) => {
                    Err(UnimplementedFeature::ArrayInitializer.at(a.span).into())
                }
                None => Ok(()),
            }
        })
    }
    fn check_initializer(
        &mut self,
        expected_type: &Type,
        expression: &Expression,
        span: Span,
        scope: ScopeId,
    ) -> SemanticResult {
        let expression_result = self.expression(expression, scope)?;
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
