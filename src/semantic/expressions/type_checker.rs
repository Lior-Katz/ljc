use crate::ast;
use crate::error::Diagnose;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};
use ast::Expression;

pub fn type_check(expression: &Expression) -> SemanticResult {
    let span = expression.span().clone();
    match expression {
        Expression::IntegerLiteral { .. }
        | Expression::LongLiteral { .. }
        | Expression::BooleanLiteral { .. }
        | Expression::CharLiteral { .. }
        | Expression::StringLiteral { .. }
        | Expression::NullLiteral(_)
        | Expression::Name(_) => Ok(()),
        Expression::Assignment { .. } => Err(UnimplementedFeature::Assignment.at(span).into()),
        Expression::PostIncrement(_) => Err(UnimplementedFeature::PostIncrement.at(span).into()),
        Expression::PostDecrement(_) => Err(UnimplementedFeature::PostDecrement.at(span).into()),
        Expression::PreIncrement(_) => Err(UnimplementedFeature::PreIncrement.at(span).into()),
        Expression::PreDecrement(_) => Err(UnimplementedFeature::PreDecrement.at(span).into()),
        Expression::UnaryPlus(_) => Err(UnimplementedFeature::UnaryPlus.at(span).into()),
        Expression::UnaryMinus(_) => Err(UnimplementedFeature::UnaryMinus.at(span).into()),
        Expression::BitwiseComplement(_) => {
            Err(UnimplementedFeature::BitwiseComplement.at(span).into())
        }
        Expression::LogicalNot(_) => Err(UnimplementedFeature::LogicalNot.at(span).into()),
        Expression::BinaryOp { .. } => Err(UnimplementedFeature::BinaryOp.at(span).into()),
        Expression::ConditionalExpression { .. } => {
            Err(UnimplementedFeature::TernaryConditional.at(span).into())
        }
        Expression::MemberAccess(_) => Err(UnimplementedFeature::MemberAccess.at(span).into()),
        Expression::MethodCall(_) => Err(UnimplementedFeature::MethodCall.at(span).into()),
        Expression::InstanceCreation { .. } => {
            Err(UnimplementedFeature::InstanceCreation.at(span).into())
        }
        Expression::ArrayCreation { .. } => {
            Err(UnimplementedFeature::ArrayCreation.at(span).into())
        }
        Expression::ArrayAccess(_) => Err(UnimplementedFeature::ArrayAccess.at(span).into()),
        Expression::Switch(_) => Err(UnimplementedFeature::SwitchExpression.at(span).into()),
        Expression::This(_) => Err(UnimplementedFeature::This.at(span).into()),
        Expression::QualifiedThis(_) => Err(UnimplementedFeature::QualifiedThis.at(span).into()),
        Expression::ClassLiteral(_) => Err(UnimplementedFeature::ClassLiteral.at(span).into()),
        Expression::MethodReference { .. } => {
            Err(UnimplementedFeature::MethodReference.at(span).into())
        }
    }
}
