use crate::ast::{Expression, Modifiers, VariableDeclaration, VariableInitializer};
use crate::error::Diagnose;
use crate::file::Span;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceIter, SemanticResult, UnimplementedFeature};

impl SemanticAnalyzer<'_> {
    #[allow(unused_variables)]
    pub fn variable_declaration(
        &mut self,
        var_decl: &VariableDeclaration,
        modifiers: &Modifiers,
        span: Span,
    ) -> SemanticResult {
        (&var_decl.declarators).coalesce(|declarator| match &declarator.initializer {
            Some(VariableInitializer::Expression(e)) => {
                self.check_initializer(e)?;
                Ok(())
            }
            Some(VariableInitializer::ArrayInitializer(a)) => {
                Err(UnimplementedFeature::ArrayInitializer.at(a.span).into())
            }
            None => Ok(()),
        })
    }
    fn check_initializer(&self, expression: &Expression) -> SemanticResult {
        let _ = self.expression(expression)?;
        Ok(())
    }
}
