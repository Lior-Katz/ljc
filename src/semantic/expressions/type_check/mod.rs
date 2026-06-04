mod binop;
mod name_expression;

use crate::ast;
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, TypeMismatch, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::ScopeId;
use crate::semantic::types::Numeric;
use crate::semantic::types::Type;
use ast::Expression;
use std::ops::Not;

impl SemanticAnalyzer<'_> {
    pub fn type_check(
        &self,
        expression: &Expression,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        let span = expression.span().clone();
        match expression {
            Expression::IntegerLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Int.into())),
            Expression::LongLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Long.into())),
            Expression::BooleanLiteral { .. } => Ok(ExpressionResult::Value(Type::Boolean)),
            Expression::CharLiteral { .. } => Ok(ExpressionResult::Value(Numeric::Char.into())),
            Expression::NullLiteral(_) => Ok(ExpressionResult::Value(Type::Null).into()),
            Expression::StringLiteral { .. } => {
                Err(UnimplementedFeature::StringLiteral.at(span).into())
            }
            Expression::Name(name) => self.name_expression(name, scope),
            Expression::Assignment { .. } => Err(UnimplementedFeature::Assignment.at(span).into()),
            Expression::PostIncrement(e)
            | Expression::PostDecrement(e)
            | Expression::PreIncrement(e)
            | Expression::PreDecrement(e) => {
                let ty = self.check_convertible_to_numeric_type(e, AllowValue::False, scope)?;
                Ok(ExpressionResult::Value(ty))
            }
            Expression::UnaryPlus(e)
            | Expression::UnaryMinus(e)
            | Expression::BitwiseComplement(e) => {
                let ty = self.check_convertible_to_numeric_type(e, AllowValue::True, scope)?;
                Ok(ExpressionResult::Value(ty))
            }
            Expression::LogicalNot(e) => {
                self.check_primitive_or_boxed_boolean(e, scope)?;
                Ok(ExpressionResult::Value(Type::Boolean))
            }
            Expression::BinaryOp { left, right, op } => self.binary_op(left, right, op, scope),
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

    fn check_convertible_to_numeric_type(
        &self,
        expression: &Expression,
        allow_value: AllowValue,
        scope: ScopeId,
    ) -> SemanticResult<Type> {
        let eval_type = self.type_check(expression, scope)?;
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
                        .into())
                }
            }
            ExpressionResult::Void => {
                Err(TypeMismatch::VoidExpression.at(*expression.span()).into())
            } // TODO: add test once function calls are implemented
        }
    }

    fn check_not_void(&self, expression: &Expression, scope: ScopeId) -> SemanticResult<Type> {
        match self.type_check(expression, scope)? {
            ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => Ok(ty),
            ExpressionResult::Void => {
                Err(TypeMismatch::VoidExpression.at(*expression.span()).into())
            }
        }
    }

    fn check_primitive_or_boxed_boolean(
        &self,
        expression: &Expression,
        scope: ScopeId,
    ) -> SemanticResult {
        let eval_type = self.type_check(expression, scope)?;
        match eval_type {
            ExpressionResult::Void => {
                Err(TypeMismatch::VoidExpression.at(*expression.span()).into())
            }
            ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
                if ty.is_primitive_or_boxed_boolean() {
                    Ok(())
                } else {
                    Err(TypeMismatch::NonBooleanOperand
                        .at(*expression.span())
                        .into())
                }
            }
        }
    }
}

enum AllowValue {
    True,
    False,
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
