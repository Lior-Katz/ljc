use crate::ast::Expression;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::SemanticResult;

mod type_checker;

impl SemanticAnalyzer {
    pub(super) fn expression(&mut self, expression: &Expression) -> SemanticResult {
        type_checker::type_check(expression)
    }
}
