use crate::ast::{BinOp, Expression};
use crate::error::Diagnose;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{CoalesceRes, Fold, SemanticResult, TypeMismatch};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::expressions::type_check::AllowValue;
use crate::semantic::symbol_table::ScopeId;
use crate::semantic::types::{
    IntegralMaybeBoxed, NumericContext, Primitive, binary_numeric_promotion,
    unary_numeric_promotion,
};

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
            BinOp::Multiply | BinOp::Divide | BinOp::Modulo | BinOp::Add | BinOp::Subtract => self
                .check_convertible_to_numeric_type(left, AllowValue::True, scope)
                .fold(
                    self.check_convertible_to_numeric_type(right, AllowValue::True, scope),
                    |ty_left, ty_right| {
                        Ok(ExpressionResult::Value(
                            binary_numeric_promotion(ty_left, ty_right, NumericContext::Arithmetic)
                                .into(),
                        ))
                    },
                ),
            BinOp::Less | BinOp::LessEqual | BinOp::Greater | BinOp::GreaterEqual => {
                self.check_convertible_to_numeric_type(left, AllowValue::True, scope)
                    .map(|_| ())
                    .coalesce(
                        self.check_convertible_to_numeric_type(right, AllowValue::True, scope)
                            .map(|_| ()),
                    )?;
                Ok(ExpressionResult::Value(Primitive::Boolean.into()))
            }
            BinOp::LeftShift | BinOp::SignedRightShift | BinOp::UnsignedRightShift => {
                // TODO: add test for non-integral operands in shift expressions
                self.check_convertible_to_integral_type(left, AllowValue::True, scope)
                    .fold(
                        self.check_convertible_to_integral_type(right, AllowValue::True, scope),
                        |ty_left, _| {
                            Ok(ExpressionResult::Value(
                                unary_numeric_promotion(ty_left.into(), NumericContext::Arithmetic)
                                    .into(),
                            ))
                        },
                    )?;
                Ok(ExpressionResult::Value(Primitive::Boolean.into()))
            }
            BinOp::Equal | BinOp::NotEqual => {
                let left_type = self.check_not_void(left, scope)?;
                let right_type = self.check_not_void(right, scope)?;
                if left_type.is_convertible_to_numeric_type()
                    && right_type.is_convertible_to_numeric_type()
                {
                    Ok(ExpressionResult::Value(Primitive::Boolean.into()))
                } else if left_type.is_primitive_or_boxed_boolean()
                    && right_type.is_primitive_or_boxed_boolean()
                {
                    Ok(ExpressionResult::Value(Primitive::Boolean.into()))
                } else {
                    Err(TypeMismatch::IncompatibleEquality.at(span).into())
                }
            }
            BinOp::BitwiseAnd | BinOp::BitwiseXor | BinOp::BitwiseOr => {
                let left_type = self.check_not_void(left, scope)?;
                let right_type = self.check_not_void(right, scope)?;
                if left_type.is_primitive_or_boxed_boolean()
                    && right_type.is_primitive_or_boxed_boolean()
                {
                    Ok(ExpressionResult::Value(Primitive::Boolean.into()))
                } else if let (Some(left_type), Some(right_type)) =
                    (left_type.as_integral_maybe_boxed(), right_type.as_integral_maybe_boxed())
                {
                    Ok(ExpressionResult::Value(
                        binary_numeric_promotion(
                            left_type.into(),
                            right_type.into(),
                            NumericContext::Arithmetic,
                        )
                        .into(),
                    ))
                } else {
                    Err(TypeMismatch::BitwiseOpIncompatibleType.at(span).into())
                }
            }
            BinOp::LogicalAnd | BinOp::LogicalOr => {
                self.check_primitive_or_boxed_boolean(left, scope)
                    .coalesce(self.check_primitive_or_boxed_boolean(right, scope))?;
                Ok(ExpressionResult::Value(Primitive::Boolean.into()))
            }
        }
    }

    fn check_convertible_to_integral_type(
        &self,
        expression: &Expression,
        allow_value: AllowValue,
        scope: ScopeId,
    ) -> SemanticResult<IntegralMaybeBoxed> {
        let eval_type = self.type_check(expression, scope)?;
        match eval_type {
            ExpressionResult::Value(_) if !allow_value => Err(TypeMismatch::NeedVariableFoundValue
                .at(*expression.span())
                .into()),
            ExpressionResult::Value(ty) | ExpressionResult::Variable(ty) => {
                // TODO: add test once float/double literals are implemented, or variables/parameters
                ty.as_integral_maybe_boxed().ok_or(
                    TypeMismatch::NonIntegralOperand
                        .at(*expression.span())
                        .into(),
                )
            }
            ExpressionResult::Void => {
                Err(TypeMismatch::VoidExpression.at(*expression.span()).into())
            } // TODO: add test once function calls are implemented
        }
    }
}
