use crate::ast::Expression;
use crate::semantic::error::SemanticResult;

mod type_checker;

pub fn analyze_expression(expression: &Expression) -> SemanticResult {
    type_checker::type_check(expression)
}
