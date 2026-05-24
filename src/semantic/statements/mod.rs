use crate::ast::Statement;
use crate::semantic::error::SemanticResult;
use crate::semantic::expressions::analyze_expression;

pub fn statement(statement: &Statement) -> SemanticResult {
    match statement {
        Statement::ExpressionStatement(e) => analyze_expression(e),
        _ => todo!("Only expression statements are supported for now"),
    }
}
