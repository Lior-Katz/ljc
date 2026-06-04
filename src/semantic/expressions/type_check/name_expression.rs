use crate::ast::Identifier;
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{Error, NameResolutionKind, SemanticResult, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::{Entity, ScopeId};

impl SemanticAnalyzer<'_> {
    pub(super) fn name_expression(
        &self,
        name: &Identifier,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        let span = name.span;

        match self.symbol_table.lookup(&name.value, scope) {
            None => Err(Error::UnknownSymbol(name.value.clone()).at(span).into()),
            Some(Entity::Variable(var)) => {
                Ok(ExpressionResult::Variable(self.attributes.get(*var).unwrap().ty.clone()))
            }
            Some(Entity::Type(_)) => {
                Err(Error::ExpressionNameNotVariable(name.value.clone(), NameResolutionKind::Type)
                    .at(span)
                    .into())
            }
            Some(Entity::Method(_)) => Err(Error::ExpressionNameNotVariable(
                name.value.clone(),
                NameResolutionKind::Method,
            )
            .at(span)
            .into()),
            Some(Entity::Field(_)) => {
                Err(UnimplementedFeature::FieldAccessSimpleName.at(span).into())
            }
        }
    }
}
