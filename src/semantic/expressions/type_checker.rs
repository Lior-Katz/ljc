use crate::ast;
use crate::error::Diagnose;
use crate::semantic::error::{SemanticResult, TypeMismatch, UnimplementedFeature};
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
        Expression::PostIncrement(e)
        | Expression::PostDecrement(e)
        | Expression::PreIncrement(e)
        | Expression::PreDecrement(e) => {
            let eval_type = e.evaluation_type()?;
            let span = e.span().clone();
            match eval_type {
                ExpressionResult::Variable(ty) => {
                    if ty.is_convertible_to_numeric_type() {
                        Ok(())
                    } else {
                        Err(TypeMismatch::NonNumericOperand.at(span).into()) // TODO: add test
                    }
                }
                ExpressionResult::Void => Err(TypeMismatch::VoidExpression.at(span).into()), // TODO: add test
                ExpressionResult::Value(_) => {
                    Err(TypeMismatch::NeedVariableFoundValue.at(span).into())
                }
            }
        }
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

trait ExpressionNode {
    fn evaluation_type(&self) -> SemanticResult<ExpressionResult>;
}

impl ExpressionNode for Expression {
    fn evaluation_type(&self) -> SemanticResult<ExpressionResult> {
        let span = self.span().clone();
        match self {
            Expression::IntegerLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Int.into())),
            Expression::LongLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Long.into())),
            Expression::BooleanLiteral { .. } => Ok(ExpressionResult::Value(Type::Boolean)),
            Expression::CharLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Char.into())),
            Expression::StringLiteral { .. } => {
                Err(UnimplementedFeature::StringLiteral.at(span).into())
            }
            Expression::NullLiteral(_) => Ok(ExpressionResult::Value(Type::Null)),
            Expression::Name(_) => Err(UnimplementedFeature::NameExpression.at(span).into()),
            Expression::Assignment { .. } => Err(UnimplementedFeature::Assignment.at(span).into()),
            Expression::PostIncrement(_) => Err(UnimplementedFeature::PostIncrementAsSubExpression
                .at(span)
                .into()),
            Expression::PostDecrement(_) => Err(UnimplementedFeature::PostDecrementAsSubExpression
                .at(span)
                .into()),
            Expression::PreIncrement(_) => Err(UnimplementedFeature::PreIncrementAsSubExpression
                .at(span)
                .into()),
            Expression::PreDecrement(_) => Err(UnimplementedFeature::PreDecrementAsSubExpression
                .at(span)
                .into()),
            Expression::BitwiseComplement(_) => {
                Err(UnimplementedFeature::BitwiseComplement.at(span).into())
            }
            Expression::LogicalNot(_) => Err(UnimplementedFeature::LogicalNot.at(span).into()),
            Expression::UnaryPlus(_) => Err(UnimplementedFeature::UnaryPlus.at(span).into()),
            Expression::UnaryMinus(_) => Err(UnimplementedFeature::UnaryMinus.at(span).into()),
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
            Expression::QualifiedThis(_) => {
                Err(UnimplementedFeature::QualifiedThis.at(span).into())
            }
            Expression::ClassLiteral(_) => Err(UnimplementedFeature::ClassLiteral.at(span).into()),
            Expression::MethodReference { .. } => {
                Err(UnimplementedFeature::MethodReference.at(span).into())
            }
        }
    }
}

#[allow(dead_code)]
enum ExpressionResult {
    Void,
    Value(Type),
    Variable(Type),
}

#[allow(dead_code)]
enum Type {
    Numeric(Numeric),
    Boolean,
    Null,
}

#[allow(dead_code)]
enum Numeric {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
}

impl Type {
    /// [§5.1.8](https://docs.oracle.com/javase/specs/jls/se26/html/jls-5.html#jls-5.1.8) - A type is said to be convertible to a numeric type if it is a numeric type (§4.2), or it is a reference type that may be converted to a numeric type by unboxing conversion.
    pub fn is_convertible_to_numeric_type(&self) -> bool {
        self.is_numeric()
        // TODO: include unboxing conversion
    }

    /// [§4.2](https://docs.oracle.com/javase/specs/jls/se26/html/jls-4.html#jls-4.2) - The numeric types are the integral types and the floating-point types.
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Numeric(_) => true,
            Self::Boolean | Self::Null => false,
        }
    }
}

impl From<Numeric> for Type {
    fn from(value: Numeric) -> Self {
        Self::Numeric(value)
    }
}
