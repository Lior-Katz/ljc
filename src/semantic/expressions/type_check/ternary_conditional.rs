use crate::ast::Expression;
use crate::error::Diagnose;
use crate::file::Span;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::error::{SemanticResult, UnimplementedFeature};
use crate::semantic::expressions::ExpressionResult;
use crate::semantic::symbol_table::ScopeId;
use crate::semantic::types::{
    BooleanMaybeBoxed, NumericContext, NumericMaybeBoxed, Type, binary_numeric_promotion,
};

impl SemanticAnalyzer<'_> {
    pub(super) fn ternary_conditional(
        &mut self,
        condition: &Expression,
        if_true: &Expression,
        if_false: &Expression,
        scope: ScopeId,
    ) -> SemanticResult<ExpressionResult> {
        self.check_primitive_or_boxed_boolean(condition, scope)?;
        let true_branch_type = self.check_not_void(if_true, scope)?;
        let false_branch_type = self.check_not_void(if_false, scope)?;

        if let (Some(t), Some(f)) = (
            true_branch_type.as_boolean_maybe_boxed(),
            false_branch_type.as_boolean_maybe_boxed(),
        ) {
            Ok(ExpressionResult::Value(boolean_conditional_expression(t, f).into()))
        } else if let (Some(t), Some(f)) = (
            true_branch_type.as_numeric_maybe_boxed(),
            false_branch_type.as_numeric_maybe_boxed(),
        ) {
            Ok(ExpressionResult::Value(numeric_conditional_expression(t, f).into()))
        } else {
            reference_conditional_expression(true_branch_type, false_branch_type, *condition.span())
        }
    }
}

fn boolean_conditional_expression(t: BooleanMaybeBoxed, f: BooleanMaybeBoxed) -> BooleanMaybeBoxed {
    match (t, f) {
        (BooleanMaybeBoxed::Boxed, BooleanMaybeBoxed::Boxed) => BooleanMaybeBoxed::Boxed,
        (BooleanMaybeBoxed::Primitive, BooleanMaybeBoxed::Primitive)
        | (BooleanMaybeBoxed::Primitive, BooleanMaybeBoxed::Boxed)
        | (BooleanMaybeBoxed::Boxed, BooleanMaybeBoxed::Primitive) => BooleanMaybeBoxed::Primitive,
    }
}

fn numeric_conditional_expression(
    true_branch_type: NumericMaybeBoxed,
    false_branch_type: NumericMaybeBoxed,
) -> NumericMaybeBoxed {
    binary_numeric_promotion(true_branch_type, false_branch_type, NumericContext::Choice).into()
}
fn reference_conditional_expression(
    true_branch_type: Type,
    false_branch_type: Type,
    span: Span,
) -> SemanticResult<ExpressionResult> {
    if true_branch_type == false_branch_type {
        Ok(ExpressionResult::Value(true_branch_type))
    } else if true_branch_type == Type::Null {
        Ok(ExpressionResult::Value(false_branch_type))
    } else if false_branch_type == Type::Null {
        Ok(ExpressionResult::Value(true_branch_type))
    } else {
        Err(UnimplementedFeature::TernaryConditional2ReferenceTypes
            .at(span)
            .into())
    }
}

#[cfg(test)]
mod tests {
    use crate::semantic::expressions::type_check::ternary_conditional::{
        boolean_conditional_expression, numeric_conditional_expression,
    };
    use crate::semantic::types::{BooleanMaybeBoxed, Numeric, NumericBoxed, NumericMaybeBoxed};

    macro_rules! assert {
        (
            expected = $expected:expr,
            t = $t_ty:expr,
            f = $f_ty:expr
        ) => {
            assert_eq!(
                NumericMaybeBoxed::from($expected),
                numeric_conditional_expression($t_ty.into(), $f_ty.into())
            )
        };
    }

    macro_rules! boxed_or_unboxed {
        (
            expected = $expected:expr,
            primitive1 = $primitive1:expr,
            boxed1 = $boxed1:expr,
            primitive2 = $primitive2:expr,
            boxed2 = $boxed2:expr
        ) => {
            assert!(expected = $expected, t = $primitive1, f = $primitive2);
            assert!(expected = $expected, t = $primitive1, f = $boxed2);
            assert!(expected = $expected, t = $boxed1, f = $primitive2);
            assert!(expected = $expected, t = $boxed1, f = $boxed2);
            assert!(expected = $expected, t = $primitive2, f = $primitive1);
            assert!(expected = $expected, t = $primitive2, f = $boxed1);
            assert!(expected = $expected, t = $boxed2, f = $primitive1);
            assert!(expected = $expected, t = $boxed2, f = $boxed1);
        };
    }

    #[test]
    fn same_type_stays() {
        // TODO: add test for same boxed type once Type can hold them
        macro_rules! same_type {
            ($ty:expr) => {
                assert!(expected = $ty, t = $ty, f = $ty);
            };
        }
        same_type!(Numeric::Byte);
        same_type!(Numeric::Short);
        same_type!(Numeric::Int);
        same_type!(Numeric::Long);
        same_type!(Numeric::Char);
        same_type!(Numeric::Float);
        same_type!(Numeric::Double);
    }

    #[test]
    fn boxing_conversion() {
        macro_rules! unboxing_conversion {
            ($primitive:expr, $boxed:expr) => {
                assert!(expected = $primitive, t = $primitive, f = $boxed);
                assert!(expected = $primitive, t = $boxed, f = $primitive);
            };
        }

        unboxing_conversion!(Numeric::Byte, NumericBoxed::Byte);
        unboxing_conversion!(Numeric::Short, NumericBoxed::Short);
        unboxing_conversion!(Numeric::Int, NumericBoxed::Integer);
        unboxing_conversion!(Numeric::Long, NumericBoxed::Long);
        unboxing_conversion!(Numeric::Char, NumericBoxed::Character);
        unboxing_conversion!(Numeric::Float, NumericBoxed::Float);
        unboxing_conversion!(Numeric::Double, NumericBoxed::Double);
    }

    #[test]
    fn byte_to_short_widening_conversion() {
        boxed_or_unboxed!(
            expected = Numeric::Short,
            primitive1 = Numeric::Byte,
            boxed1 = NumericBoxed::Byte,
            primitive2 = Numeric::Short,
            boxed2 = NumericBoxed::Short
        );
    }

    #[test]
    fn boolean_conditional() {
        assert_eq!(
            BooleanMaybeBoxed::Primitive,
            boolean_conditional_expression(
                BooleanMaybeBoxed::Primitive,
                BooleanMaybeBoxed::Primitive
            )
        );
        assert_eq!(
            BooleanMaybeBoxed::Primitive,
            boolean_conditional_expression(BooleanMaybeBoxed::Boxed, BooleanMaybeBoxed::Primitive)
        );
        assert_eq!(
            BooleanMaybeBoxed::Primitive,
            boolean_conditional_expression(BooleanMaybeBoxed::Primitive, BooleanMaybeBoxed::Boxed)
        );
        assert_eq!(
            BooleanMaybeBoxed::Boxed,
            boolean_conditional_expression(BooleanMaybeBoxed::Boxed, BooleanMaybeBoxed::Boxed)
        );
    }
}
