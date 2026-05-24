use crate::ast;
use crate::semantic::error::SemanticResult;
use ast::Expression;

pub fn type_check(expression: &Expression) -> SemanticResult {
    match expression {
        Expression::IntegerLiteral { .. }
        | Expression::LongLiteral { .. }
        | Expression::BooleanLiteral { .. }
        | Expression::CharLiteral { .. }
        | Expression::StringLiteral { .. }
        | Expression::NullLiteral(_)
        | Expression::Name(_) => Ok(()),
        _ => todo!("Type check for {:?}", expression),
    }
}
