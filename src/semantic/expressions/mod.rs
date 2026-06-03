use crate::ast::Expression;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::SemanticResult;
use crate::semantic::symbol_table::ScopeId;
use crate::semantic::types::Type;

mod type_checker;

impl SemanticAnalyzer<'_> {
    pub(super) fn expression(&self, expression: &Expression, scope: ScopeId) -> SemanticResult<ExpressionResult> {
        self.type_check(expression, scope)
    }
}

pub enum ExpressionResult {
    Void,
    Value(Type),
    Variable(Type),
}
