use crate::ast;
use crate::ast::BinOp;
use crate::error::Diagnose;
use crate::semantic::error::{
    CoalesceRes, Fold, SemanticResult, TypeMismatch, UnimplementedFeature,
};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::types::Numeric;
use crate::semantic::types::Type;
use ast::Expression;
use std::ops::Not;

pub fn type_check(expression: &Expression) -> SemanticResult<ExpressionResult> {
    let span = expression.span().clone();
    // TODO: add tests for non-numeric variables in arithmetic operations once names are possible
    match expression {
        Expression::IntegerLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Int.into())),
        Expression::LongLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Long.into())),
        Expression::BooleanLiteral { .. } => Ok(ExpressionResult::Value(Type::Boolean)),
        Expression::CharLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Char.into())),
        Expression::NullLiteral(_) => Ok(ExpressionResult::Value(Type::Null).into()),
        Expression::StringLiteral { .. } => {
            Err(UnimplementedFeature::StringLiteral.at(span).into())
        }
        Expression::Name(_) => Err(UnimplementedFeature::NameExpression.at(span).into()),
        Expression::Assignment { .. } => Err(UnimplementedFeature::Assignment.at(span).into()),
        Expression::PostIncrement(e)
        | Expression::PostDecrement(e)
        | Expression::PreIncrement(e)
        | Expression::PreDecrement(e) => {
            let ty = check_convertible_to_numeric_type(e, AllowValue::False)?;
            Ok(ExpressionResult::Value(ty))
        }
        Expression::UnaryPlus(e) | Expression::UnaryMinus(e) | Expression::BitwiseComplement(e) => {
            let ty = check_convertible_to_numeric_type(e, AllowValue::True)?;
            Ok(ExpressionResult::Value(ty))
        }
        Expression::LogicalNot(e) => {
            let eval_type = type_check(e)?;
            match eval_type {
                ExpressionResult::Void => Err(TypeMismatch::VoidExpression.at(*e.span()).into()),
                ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
                    if ty.is_primitive_or_boxed_boolean() {
                        Ok(ExpressionResult::Value(Type::Boolean))
                    } else {
                        Err(TypeMismatch::NonBooleanOperand.at(*e.span()).into())
                    }
                }
            }
        }
        Expression::BinaryOp { left, right, op } => match op {
            BinOp::Multiply(_)
            | BinOp::Divide(_)
            | BinOp::Modulo(_)
            | BinOp::Add(_)
            | BinOp::Subtract(_) => check_convertible_to_numeric_type(left, AllowValue::True).fold(
                check_convertible_to_numeric_type(right, AllowValue::True),
                |ty_left, ty_right| {
                    if ty_left == ty_right {
                        Ok(ExpressionResult::Value(ty_left))
                    } else {
                        Err(UnimplementedFeature::NumericPromotion
                            .at(*left.span())
                            .into())
                    }
                },
            ),
            BinOp::Less(_) | BinOp::LessEqual(_) | BinOp::Greater(_) | BinOp::GreaterEqual(_) => {
                check_convertible_to_numeric_type(left, AllowValue::True)
                    .map(|_| ())
                    .coalesce(
                        check_convertible_to_numeric_type(right, AllowValue::True).map(|_| ()),
                    )?;
                Ok(ExpressionResult::Value(Type::Boolean))
            }
            BinOp::LeftShift(_) | BinOp::SignedRightShift(_) | BinOp::UnsignedRightShift(_) => {
                check_convertible_to_integral_type(left, AllowValue::True)
                    .coalesce(check_convertible_to_integral_type(right, AllowValue::True))?;
                Ok(ExpressionResult::Value(Type::Boolean))
            }
            _ => todo!(),
        },
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

enum AllowValue {
    True,
    False,
}

fn check_convertible_to_numeric_type(
    expression: &Expression,
    allow_value: AllowValue,
) -> SemanticResult<Type> {
    let eval_type = type_check(expression)?;
    match eval_type {
        ExpressionResult::Value(_) if !allow_value => Err(TypeMismatch::NeedVariableFoundValue
            .at(*expression.span())
            .into()),
        ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
            if ty.is_convertible_to_numeric_type() {
                Ok(ty)
            } else {
                Err(TypeMismatch::NonNumericOperand
                    .at(*expression.span())
                    .into()) // TODO: add tests for variables
            }
        }
        ExpressionResult::Void => Err(TypeMismatch::VoidExpression.at(*expression.span()).into()), // TODO: add test once function calls are implemented
    }
}

fn check_convertible_to_integral_type(
    expression: &Expression,
    allow_value: AllowValue,
) -> SemanticResult {
    let eval_type = type_check(expression)?;
    match eval_type {
        ExpressionResult::Value(_) if !allow_value => Err(TypeMismatch::NeedVariableFoundValue
            .at(*expression.span())
            .into()),
        ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
            if ty.is_convertible_to_integral_type() {
                // TODO: add test once float/double literals are implemented, or variables/parameters
                Ok(())
            } else {
                Err(TypeMismatch::NonIntegralOperand
                    .at(*expression.span())
                    .into()) // TODO: add tests for variables
            }
        }
        ExpressionResult::Void => Err(TypeMismatch::VoidExpression.at(*expression.span()).into()), // TODO: add test once function calls are implemented
    }
}

impl Not for AllowValue {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self {
            AllowValue::True => false,
            AllowValue::False => true,
        }
    }
}
