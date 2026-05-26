use crate::ast;
use crate::ast::BinOp;
use crate::error::Diagnose;
use crate::semantic::error::{CoalesceRes, SemanticResult, TypeMismatch, UnimplementedFeature};
use ast::Expression;
use std::ops::Not;

pub fn type_check(expression: &Expression) -> SemanticResult {
    let span = expression.span().clone();
    // TODO: add tests for non-numeric variables in arithmetic operations once names are possible
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
        | Expression::PreDecrement(e) => check_convertible_to_numeric_type(e, AllowValue::False),
        Expression::UnaryPlus(e) | Expression::UnaryMinus(e) | Expression::BitwiseComplement(e) => {
            check_convertible_to_numeric_type(e, AllowValue::True)
        }
        Expression::LogicalNot(e) => {
            let eval_type = e.evaluation_type()?;
            match eval_type {
                ExpressionResult::Void => Err(TypeMismatch::VoidExpression.at(*e.span()).into()),
                ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
                    if ty.is_primitive_or_boxed_boolean() {
                        Ok(())
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
            | BinOp::Subtract(_)
            | BinOp::Less(_)
            | BinOp::LessEqual(_)
            | BinOp::Greater(_)
            | BinOp::GreaterEqual(_) => check_convertible_to_numeric_type(left, AllowValue::True)
                .map(|_| ())
                .coalesce(check_convertible_to_numeric_type(right, AllowValue::True).map(|_| ())),
            BinOp::LeftShift(_) | BinOp::SignedRightShift(_) | BinOp::UnsignedRightShift(_) => {
                check_convertible_to_integral_type(left, AllowValue::True)
                    .coalesce(check_convertible_to_integral_type(right, AllowValue::True))
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
                Err(UnimplementedFeature::BitwiseComplementAsSubExpression
                    .at(span)
                    .into())
            }
            Expression::LogicalNot(_) => Err(UnimplementedFeature::LogicalNot.at(span).into()),
            Expression::UnaryPlus(_) => Err(UnimplementedFeature::UnaryPlusInSubExpression
                .at(span)
                .into()),
            Expression::UnaryMinus(_) => Err(UnimplementedFeature::UnaryMinusInSubExpression
                .at(span)
                .into()),
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

enum AllowValue {
    True,
    False,
}

fn check_convertible_to_numeric_type(
    expression: &Expression,
    allow_value: AllowValue,
) -> SemanticResult {
    let eval_type = expression.evaluation_type()?;
    match eval_type {
        ExpressionResult::Value(_) if !allow_value => Err(TypeMismatch::NeedVariableFoundValue
            .at(*expression.span())
            .into()),
        ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
            if ty.is_convertible_to_numeric_type() {
                Ok(())
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
    let eval_type = expression.evaluation_type()?;
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
    fn is_convertible_to_numeric_type(&self) -> bool {
        self.is_numeric()
        // TODO: include unboxing conversion
    }

    /// [§4.2](https://docs.oracle.com/javase/specs/jls/se26/html/jls-4.html#jls-4.2) - The numeric types are the integral types and the floating-point types.
    fn is_numeric(&self) -> bool {
        match self {
            Self::Numeric(_) => true,
            Self::Boolean | Self::Null => false,
        }
    }

    fn is_convertible_to_integral_type(&self) -> bool {
        self.is_integral()
        // TODO: include unboxing conversion
    }

    fn is_integral(&self) -> bool {
        match self {
            Type::Numeric(numeric) => numeric.is_integral(),
            Type::Boolean | Type::Null => false,
        }
    }

    #[allow(non_snake_case)]
    fn is_primitive_or_boxed_boolean(&self) -> bool {
        matches!(self, Self::Boolean)
        // TODO: check for boxed Boolean as well
    }
}

impl Numeric {
    fn is_integral(&self) -> bool {
        match self {
            Numeric::Byte | Numeric::Short | Numeric::Int | Numeric::Long | Numeric::Char => true,
            Numeric::Float | Numeric::Double => false,
        }
    }
}

impl From<Numeric> for Type {
    fn from(value: Numeric) -> Self {
        Self::Numeric(value)
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
