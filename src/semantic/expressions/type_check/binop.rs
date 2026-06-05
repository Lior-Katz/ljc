use crate::ast::{BinOp, Expression};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{
    CoalesceRes, Fold, SemanticResult, TypeMismatch, UnimplementedFeature,
};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::expressions::type_check::AllowValue;
use crate::semantic::symbol_table::ScopeId;
use crate::semantic::types::Type;

impl SemanticAnalyzer<'_> {
    pub(super) fn binary_op(
        &self,
        left: &Box<Expression>,
        right: &Box<Expression>,
        op: &BinOp,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        let span = *left.span();
        match op {
            BinOp::Multiply(_)
            | BinOp::Divide(_)
            | BinOp::Modulo(_)
            | BinOp::Add(_)
            | BinOp::Subtract(_) => self
                .check_convertible_to_numeric_type(left, AllowValue::True, scope)
                .fold(
                    self.check_convertible_to_numeric_type(right, AllowValue::True, scope),
                    |ty_left, ty_right| {
                        if ty_left == ty_right {
                            Ok(ExpressionResult::Value(ty_left.into()))
                        } else {
                            Err(UnimplementedFeature::NumericPromotion
                                .at(*left.span())
                                .into())
                        }
                    },
                ),
            BinOp::Less(_) | BinOp::LessEqual(_) | BinOp::Greater(_) | BinOp::GreaterEqual(_) => {
                self.check_convertible_to_numeric_type(left, AllowValue::True, scope)
                    .map(|_| ())
                    .coalesce(
                        self.check_convertible_to_numeric_type(right, AllowValue::True, scope)
                            .map(|_| ()),
                    )?;
                Ok(ExpressionResult::Value(Type::Boolean))
            }
            BinOp::LeftShift(_) | BinOp::SignedRightShift(_) | BinOp::UnsignedRightShift(_) => {
                self.check_convertible_to_integral_type(left, AllowValue::True, scope)
                    .coalesce(self.check_convertible_to_integral_type(
                        right,
                        AllowValue::True,
                        scope,
                    ))?;
                Ok(ExpressionResult::Value(Type::Boolean))
            }
            BinOp::Equal(_) | BinOp::NotEqual(_) => {
                let left_type = self.check_not_void(left, scope)?;
                let right_type = self.check_not_void(right, scope)?;
                if left_type.is_convertible_to_numeric_type()
                    && right_type.is_convertible_to_numeric_type()
                {
                    Ok(ExpressionResult::Value(Type::Boolean))
                } else if left_type.is_primitive_or_boxed_boolean()
                    && right_type.is_primitive_or_boxed_boolean()
                {
                    Ok(ExpressionResult::Value(Type::Boolean))
                } else {
                    Err(TypeMismatch::IncompatibleEquality.at(span).into())
                }
            }
            BinOp::BitwiseAnd(_) | BinOp::BitwiseXor(_) | BinOp::BitwiseOr(_) => {
                let left_type = self.check_not_void(left, scope)?;
                let right_type = self.check_not_void(right, scope)?;
                if left_type.is_primitive_or_boxed_boolean()
                    && right_type.is_primitive_or_boxed_boolean()
                {
                    Ok(ExpressionResult::Value(Type::Boolean))
                } else if left_type.is_convertible_to_integral_type()
                    && right_type.is_convertible_to_integral_type()
                {
                    Err(UnimplementedFeature::NumericPromotion.at(span).into())
                } else {
                    Err(TypeMismatch::BitwiseOpIncompatibleType.at(span).into())
                }
            }
            BinOp::LogicalAnd(_) | BinOp::LogicalOr(_) => {
                self.check_primitive_or_boxed_boolean(left, scope)
                    .coalesce(self.check_primitive_or_boxed_boolean(right, scope))?;
                Ok(ExpressionResult::Value(Type::Boolean))
            }
        }
    }

    fn check_convertible_to_integral_type(
        &self,
        expression: &Expression,
        allow_value: AllowValue,
        scope: ScopeId,
    ) -> SemanticResult {
        let eval_type = self.type_check(expression, scope)?;
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
                        .into())
                }
            }
            ExpressionResult::Void => {
                Err(TypeMismatch::VoidExpression.at(*expression.span()).into())
            } // TODO: add test once function calls are implemented
        }
    }
}
