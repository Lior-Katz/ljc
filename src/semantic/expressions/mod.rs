use crate::ast::Expression;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::SemanticResult;
use crate::semantic::types::Type;

mod type_checker;

impl SemanticAnalyzer {
    pub(super) fn expression(&self, expression: &Expression) -> SemanticResult<ExpressionResult> {
        type_checker::type_check(expression)
    }
}

#[allow(dead_code)]
pub enum ExpressionResult {
    Void,
    Value(Type),
    Variable(Type),
}
