use crate::ast::{Modified, Modifiers, Statement, VariableDeclaration, VariableInitializer};
use crate::error::Diagnose;
use crate::file::Span;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceIter, CoalesceRes, SemanticResult, UnimplementedFeature};

impl SemanticAnalyzer {
    pub(super) fn statement(&mut self, statement: &Statement) -> SemanticResult {
        let span = statement.span().clone();
        match statement {
            Statement::EmptyStatement(_) => {
                Err(UnimplementedFeature::EmptyStatement.at(span).into())
            }
            Statement::ExpressionStatement(e) => self.expression(e),
            Statement::Block(_) => Err(UnimplementedFeature::Block.at(span).into()),
            Statement::VariableDeclaration(Modified { item: var_decl, modifiers }) => {
                self.variable_declaration(var_decl, modifiers, span)
            }
            Statement::If { .. } => Err(UnimplementedFeature::If.at(span).into()),
            Statement::While { .. } => Err(UnimplementedFeature::While.at(span).into()),
            Statement::For { .. } => Err(UnimplementedFeature::For.at(span).into()),
            Statement::ForEach { .. } => Err(UnimplementedFeature::ForEach.at(span).into()),
            Statement::DoWhile { .. } => Err(UnimplementedFeature::DoWhile.at(span).into()),
            Statement::Labeled { .. } => Err(UnimplementedFeature::Labeled.at(span).into()),
            Statement::Break { .. } => Err(UnimplementedFeature::Break.at(span).into()),
            Statement::Continue { .. } => Err(UnimplementedFeature::Continue.at(span).into()),
            Statement::Assert { .. } => Err(UnimplementedFeature::Assert.at(span).into()),
            Statement::Return { .. } => Err(UnimplementedFeature::Return.at(span).into()),
            Statement::Try { .. } => Err(UnimplementedFeature::Try.at(span).into()),
            Statement::Throw { .. } => Err(UnimplementedFeature::Throw.at(span).into()),
            Statement::Synchronized { .. } => {
                Err(UnimplementedFeature::Synchronized.at(span).into())
            }
            Statement::Switch(_) => Err(UnimplementedFeature::SwitchStatement.at(span).into()),
            Statement::Yield { .. } => Err(UnimplementedFeature::Yield.at(span).into()),
        }
    }

    #[allow(unused_variables)]
    fn variable_declaration(
        &mut self,
        var_decl: &VariableDeclaration,
        modifiers: &Modifiers,
        span: Span,
    ) -> SemanticResult {
        (&var_decl.declarators)
            .coalesce(|declarator| {
                match &declarator.initializer {
                    Some(VariableInitializer::Expression(e)) => {
                        self.expression(e)?;
                    }
                    _ => {}
                }
                Ok(())
            })
            .coalesce(Err(UnimplementedFeature::VariableDeclaration.at(span).into()))
    }
}
