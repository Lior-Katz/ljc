use crate::semantic::types::contexts::NumericContext;
use crate::semantic::types::numeric::{Numeric, NumericBoxed, NumericMaybeBoxed};
use crate::semantic::types::{Boxed, Primitive};

/// [§5.6](https://docs.oracle.com/javase/specs/jls/se26/html/jls-5.html#jls-5.6) - Numeric contexts and numeric promotion
pub fn unary_numeric_promotion(ty: NumericMaybeBoxed, context: NumericContext) -> Numeric {
    let ty = match ty {
        NumericMaybeBoxed::Primitive(ty) => ty,
        NumericMaybeBoxed::Boxed(ty) => numeric_unboxing_conversion(ty),
    };

    match (&ty, context) {
        (Numeric::Double | Numeric::Float | Numeric::Long, _) => ty,
        (_, NumericContext::Arithmetic | NumericContext::Array) => Numeric::Int,
        (Numeric::Byte | Numeric::Short | Numeric::Char | Numeric::Int, NumericContext::Choice) => {
            ty
        }
    }
}

pub fn binary_numeric_promotion(
    ty1: NumericMaybeBoxed,
    ty2: NumericMaybeBoxed,
    context: NumericContext,
) -> Numeric {
    // 1. unboxing conversion on any reference type
    let ty1 = match ty1 {
        NumericMaybeBoxed::Primitive(ty) => ty,
        NumericMaybeBoxed::Boxed(ty) => numeric_unboxing_conversion(ty),
    };
    let ty2 = match ty2 {
        NumericMaybeBoxed::Primitive(ty) => ty,
        NumericMaybeBoxed::Boxed(ty) => numeric_unboxing_conversion(ty),
    };

    // 2. widening/narrowing
    match (ty1, ty2, context) {
        (Numeric::Double, _, _) | (_, Numeric::Double, _) => Numeric::Double,

        (Numeric::Float, _, _) | (_, Numeric::Float, _) => Numeric::Float,

        (Numeric::Long, _, _) | (_, Numeric::Long, _) => Numeric::Long,

        (_, _, NumericContext::Arithmetic | NumericContext::Array) => Numeric::Int,

        // TODO: add narrowing conversion if there is a single int operand that can fit into a smaller type
        (Numeric::Int, _, NumericContext::Choice) | (_, Numeric::Int, _) => Numeric::Int,

        (Numeric::Short, Numeric::Short, NumericContext::Choice)
        | (Numeric::Byte, Numeric::Short, NumericContext::Choice)
        | (Numeric::Short, Numeric::Byte, NumericContext::Choice) => Numeric::Short,

        (Numeric::Byte, Numeric::Byte, NumericContext::Choice) => Numeric::Byte,

        (Numeric::Char, Numeric::Char, NumericContext::Choice) => Numeric::Char,

        (Numeric::Short | Numeric::Byte, Numeric::Char, NumericContext::Choice)
        | (Numeric::Char, Numeric::Short | Numeric::Byte, NumericContext::Choice) => Numeric::Int,
    }
}

pub fn unboxing_conversion(ty: Boxed) -> Primitive {
    match ty {
        Boxed::Numeric(numeric) => numeric_unboxing_conversion(numeric).into(),
        Boxed::Boolean => Primitive::Boolean,
    }
}

fn numeric_unboxing_conversion(ty: NumericBoxed) -> Numeric {
    match ty {
        NumericBoxed::Byte => Numeric::Byte,
        NumericBoxed::Short => Numeric::Short,
        NumericBoxed::Character => Numeric::Char,
        NumericBoxed::Integer => Numeric::Int,
        NumericBoxed::Long => Numeric::Long,
        NumericBoxed::Float => Numeric::Float,
        NumericBoxed::Double => Numeric::Double,
    }
}

pub fn boxing_conversion(ty: Primitive) -> Boxed {
    match ty {
        Primitive::Numeric(numeric) => numeric_boxing_conversion(numeric).into(),
        Primitive::Boolean => Boxed::Boolean,
    }
}

pub fn numeric_boxing_conversion(ty: Numeric) -> NumericBoxed {
    match ty {
        Numeric::Byte => NumericBoxed::Byte,
        Numeric::Short => NumericBoxed::Short,
        Numeric::Int => NumericBoxed::Integer,
        Numeric::Long => NumericBoxed::Long,
        Numeric::Char => NumericBoxed::Character,
        Numeric::Float => NumericBoxed::Float,
        Numeric::Double => NumericBoxed::Double,
    }
}

#[cfg(test)]
mod tests {
    mod binary_numeric_promotion {
        use crate::semantic::types::conversions::binary_numeric_promotion;
        use crate::semantic::types::{Numeric, NumericBoxed, NumericContext};

        // TODO: Add tests for narrowing conversion to short and byte types in Choice context, when one of the operands is int but can fit into smaller type

        macro_rules! assert_choice {
            ($expected:expr, $lhs:expr, $rhs:expr) => {
                assert!($expected, $lhs, $rhs, NumericContext::Choice);
            };
        }

        macro_rules! assert {
            ($expected:expr, $lhs:expr, $rhs:expr, $numeric_context:expr) => {
                assert_eq!(
                    $expected,
                    binary_numeric_promotion($lhs.into(), $rhs.into(), $numeric_context,)
                );
            };
        }

        macro_rules! boxed_or_unboxed {
            (
                expected = $expected:expr,
                primitive1 = $primitive1:expr,
                boxed1 = $boxed1:expr,
                primitive2 = $primitive2:expr,
                boxed2 = $boxed2:expr,
                numeric_context = $numeric_context:expr
            ) => {
                assert!($expected, $primitive1, $primitive2, $numeric_context);
                assert!($expected, $primitive1, $boxed2, $numeric_context);
                assert!($expected, $boxed1, $primitive2, $numeric_context);
                assert!($expected, $boxed1, $boxed2, $numeric_context);
                assert!($expected, $primitive2, $primitive1, $numeric_context);
                assert!($expected, $primitive2, $boxed1, $numeric_context);
                assert!($expected, $boxed2, $primitive1, $numeric_context);
                assert!($expected, $boxed2, $boxed1, $numeric_context);
            };
        }

        #[test]
        fn unboxing_conversion_choice_context() {
            macro_rules! assert_unboxing {
                (
                    primitive = $primitive:expr,
                    boxed = $boxed:expr
                ) => {
                    assert_choice!($primitive, $primitive, $primitive);
                    assert_choice!($primitive, $primitive, $boxed);
                    assert_choice!($primitive, $boxed, $primitive);
                    assert_choice!($primitive, $boxed, $boxed);
                };
            }
            assert_unboxing!(primitive = Numeric::Short, boxed = NumericBoxed::Short);
            assert_unboxing!(primitive = Numeric::Byte, boxed = NumericBoxed::Byte);
            assert_unboxing!(primitive = Numeric::Char, boxed = NumericBoxed::Character);
            assert_unboxing!(primitive = Numeric::Int, boxed = NumericBoxed::Integer);
            assert_unboxing!(primitive = Numeric::Long, boxed = NumericBoxed::Long);
            assert_unboxing!(primitive = Numeric::Float, boxed = NumericBoxed::Float);
            assert_unboxing!(primitive = Numeric::Double, boxed = NumericBoxed::Double);
        }

        #[test]
        fn unboxing_conversion_array_or_arithmetic_context() {
            macro_rules! assert_unboxing {
                (
                    primitive = $primitive:expr,
                    boxed = $boxed:expr,
                    numeric_context = $numeric_context:expr
                ) => {
                    assert!($primitive, $primitive, $primitive, $numeric_context);
                    assert!($primitive, $primitive, $boxed, $numeric_context);
                    assert!($primitive, $boxed, $primitive, $numeric_context);
                    assert!($primitive, $boxed, $boxed, $numeric_context);
                };
            }
            macro_rules! test_unboxing_conversion {
                ($numeric_context:expr) => {
                    assert_unboxing!(
                        primitive = Numeric::Int,
                        boxed = NumericBoxed::Integer,
                        numeric_context = $numeric_context
                    );
                    assert_unboxing!(
                        primitive = Numeric::Long,
                        boxed = NumericBoxed::Long,
                        numeric_context = $numeric_context
                    );
                    assert_unboxing!(
                        primitive = Numeric::Float,
                        boxed = NumericBoxed::Float,
                        numeric_context = $numeric_context
                    );
                    assert_unboxing!(
                        primitive = Numeric::Double,
                        boxed = NumericBoxed::Double,
                        numeric_context = $numeric_context
                    );
                };
            }
            test_unboxing_conversion!(NumericContext::Arithmetic);
            test_unboxing_conversion!(NumericContext::Array);
        }

        #[test]
        fn double_dominates() {
            macro_rules! _boxed_or_unboxed {
                (
                    primitive = $primitive:expr,
                    boxed = $boxed:expr,
                    numeric_context = $numeric_context:expr
                ) => {
                    boxed_or_unboxed!(
                        expected = Numeric::Double,
                        primitive1 = Numeric::Double,
                        boxed1 = NumericBoxed::Double,
                        primitive2 = $primitive,
                        boxed2 = $boxed,
                        numeric_context = $numeric_context
                    );
                };
            }

            macro_rules! for_context {
                ($numeric_context:expr) => {
                    _boxed_or_unboxed!(
                        primitive = Numeric::Byte,
                        boxed = NumericBoxed::Byte,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Short,
                        boxed = NumericBoxed::Short,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Int,
                        boxed = NumericBoxed::Integer,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Long,
                        boxed = NumericBoxed::Long,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Char,
                        boxed = NumericBoxed::Character,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Float,
                        boxed = NumericBoxed::Float,
                        numeric_context = $numeric_context
                    );
                };
            }
            for_context!(NumericContext::Arithmetic);
            for_context!(NumericContext::Array);
            for_context!(NumericContext::Choice);
        }

        #[test]
        fn float_dominates() {
            macro_rules! _boxed_or_unboxed {
                (
                    primitive = $primitive:expr,
                    boxed = $boxed:expr,
                    numeric_context = $numeric_context:expr
                ) => {
                    boxed_or_unboxed!(
                        expected = Numeric::Float,
                        primitive1 = Numeric::Float,
                        boxed1 = NumericBoxed::Float,
                        primitive2 = $primitive,
                        boxed2 = $boxed,
                        numeric_context = $numeric_context
                    );
                };
            }

            macro_rules! for_context {
                ($numeric_context:expr) => {
                    _boxed_or_unboxed!(
                        primitive = Numeric::Byte,
                        boxed = NumericBoxed::Byte,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Short,
                        boxed = NumericBoxed::Short,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Int,
                        boxed = NumericBoxed::Integer,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Long,
                        boxed = NumericBoxed::Long,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Char,
                        boxed = NumericBoxed::Character,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Float,
                        boxed = NumericBoxed::Float,
                        numeric_context = $numeric_context
                    );
                };
            }
            for_context!(NumericContext::Arithmetic);
            for_context!(NumericContext::Array);
            for_context!(NumericContext::Choice);
        }

        #[test]
        fn long_dominates() {
            macro_rules! _boxed_or_unboxed {
                (
                    primitive = $primitive:expr,
                    boxed = $boxed:expr,
                    numeric_context = $numeric_context:expr
                ) => {
                    boxed_or_unboxed!(
                        expected = Numeric::Long,
                        primitive1 = Numeric::Long,
                        boxed1 = NumericBoxed::Long,
                        primitive2 = $primitive,
                        boxed2 = $boxed,
                        numeric_context = $numeric_context
                    );
                };
            }

            macro_rules! for_context {
                ($numeric_context:expr) => {
                    _boxed_or_unboxed!(
                        primitive = Numeric::Byte,
                        boxed = NumericBoxed::Byte,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Short,
                        boxed = NumericBoxed::Short,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Int,
                        boxed = NumericBoxed::Integer,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Long,
                        boxed = NumericBoxed::Long,
                        numeric_context = $numeric_context
                    );
                    _boxed_or_unboxed!(
                        primitive = Numeric::Char,
                        boxed = NumericBoxed::Character,
                        numeric_context = $numeric_context
                    );
                };
            }
            for_context!(NumericContext::Arithmetic);
            for_context!(NumericContext::Array);
            for_context!(NumericContext::Choice);
        }

        #[test]
        fn widen_to_int_in_array_or_arithmetic_context() {
            macro_rules! test_int_widening_conversion {
                ($numeric_context:expr) => {
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            Numeric::Byte.into(),
                            Numeric::Byte.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            NumericBoxed::Byte.into(),
                            NumericBoxed::Byte.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            Numeric::Short.into(),
                            Numeric::Short.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            NumericBoxed::Short.into(),
                            NumericBoxed::Short.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            Numeric::Int.into(),
                            Numeric::Int.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            NumericBoxed::Integer.into(),
                            NumericBoxed::Integer.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            Numeric::Char.into(),
                            Numeric::Char.into(),
                            $numeric_context
                        )
                    );
                    assert_eq!(
                        Numeric::Int,
                        binary_numeric_promotion(
                            NumericBoxed::Character.into(),
                            NumericBoxed::Character.into(),
                            $numeric_context
                        )
                    );
                };
            }

            test_int_widening_conversion!(NumericContext::Arithmetic);
            test_int_widening_conversion!(NumericContext::Array);
        }

        #[test]
        fn widen_to_short() {
            boxed_or_unboxed!(
                expected = Numeric::Short,
                primitive1 = Numeric::Short,
                boxed1 = NumericBoxed::Short,
                primitive2 = Numeric::Byte,
                boxed2 = NumericBoxed::Byte,
                numeric_context = NumericContext::Choice
            );
        }

        #[test]
        fn widen_to_int() {
            boxed_or_unboxed!(
                expected = Numeric::Int,
                primitive1 = Numeric::Short,
                boxed1 = NumericBoxed::Short,
                primitive2 = Numeric::Char,
                boxed2 = NumericBoxed::Character,
                numeric_context = NumericContext::Choice
            );
            boxed_or_unboxed!(
                expected = Numeric::Int,
                primitive1 = Numeric::Byte,
                boxed1 = NumericBoxed::Byte,
                primitive2 = Numeric::Char,
                boxed2 = NumericBoxed::Character,
                numeric_context = NumericContext::Choice
            );
        }
    }

    mod unary_numeric_promotion {
        use crate::semantic::types::conversions::unary_numeric_promotion;
        use crate::semantic::types::{Numeric, NumericBoxed, NumericContext};
        macro_rules! assert {
            ($expected:expr, $ty:expr, $numeric_context:expr) => {
                assert_eq!($expected, unary_numeric_promotion($ty.into(), $numeric_context));
            };
        }

        #[test]
        pub fn unboxing_conversion() {
            assert!(Numeric::Char, Numeric::Char, NumericContext::Choice);
            assert!(Numeric::Char, NumericBoxed::Character, NumericContext::Choice);

            assert!(Numeric::Byte, Numeric::Byte, NumericContext::Choice);
            assert!(Numeric::Byte, NumericBoxed::Byte, NumericContext::Choice);

            assert!(Numeric::Short, Numeric::Short, NumericContext::Choice);
            assert!(Numeric::Short, NumericBoxed::Short, NumericContext::Choice);

            assert!(Numeric::Int, Numeric::Int, NumericContext::Choice);
            assert!(Numeric::Int, NumericBoxed::Integer, NumericContext::Choice);
            assert!(Numeric::Int, Numeric::Int, NumericContext::Arithmetic);
            assert!(Numeric::Int, NumericBoxed::Integer, NumericContext::Arithmetic);
            assert!(Numeric::Int, Numeric::Int, NumericContext::Array);
            assert!(Numeric::Int, NumericBoxed::Integer, NumericContext::Array);

            assert!(Numeric::Long, Numeric::Long, NumericContext::Choice);
            assert!(Numeric::Long, NumericBoxed::Long, NumericContext::Choice);
            assert!(Numeric::Long, Numeric::Long, NumericContext::Arithmetic);
            assert!(Numeric::Long, NumericBoxed::Long, NumericContext::Arithmetic);
            assert!(Numeric::Long, Numeric::Long, NumericContext::Array);
            assert!(Numeric::Long, NumericBoxed::Long, NumericContext::Array);

            assert!(Numeric::Float, Numeric::Float, NumericContext::Choice);
            assert!(Numeric::Float, NumericBoxed::Float, NumericContext::Choice);
            assert!(Numeric::Float, Numeric::Float, NumericContext::Arithmetic);
            assert!(Numeric::Float, NumericBoxed::Float, NumericContext::Arithmetic);
            assert!(Numeric::Float, Numeric::Float, NumericContext::Array);
            assert!(Numeric::Float, NumericBoxed::Float, NumericContext::Array);

            assert!(Numeric::Double, Numeric::Double, NumericContext::Choice);
            assert!(Numeric::Double, NumericBoxed::Double, NumericContext::Choice);
            assert!(Numeric::Double, Numeric::Double, NumericContext::Arithmetic);
            assert!(Numeric::Double, NumericBoxed::Double, NumericContext::Arithmetic);
            assert!(Numeric::Double, Numeric::Double, NumericContext::Array);
            assert!(Numeric::Double, NumericBoxed::Double, NumericContext::Array);
        }

        #[test]
        pub fn widen_to_int() {
            assert!(Numeric::Int, Numeric::Char, NumericContext::Arithmetic);
            assert!(Numeric::Int, Numeric::Char, NumericContext::Array);
            assert!(Numeric::Int, NumericBoxed::Character, NumericContext::Arithmetic);
            assert!(Numeric::Int, NumericBoxed::Character, NumericContext::Array);

            assert!(Numeric::Int, Numeric::Byte, NumericContext::Arithmetic);
            assert!(Numeric::Int, Numeric::Byte, NumericContext::Array);
            assert!(Numeric::Int, NumericBoxed::Byte, NumericContext::Arithmetic);
            assert!(Numeric::Int, NumericBoxed::Byte, NumericContext::Array);

            assert!(Numeric::Int, Numeric::Short, NumericContext::Arithmetic);
            assert!(Numeric::Int, Numeric::Short, NumericContext::Array);
            assert!(Numeric::Int, NumericBoxed::Short, NumericContext::Arithmetic);
            assert!(Numeric::Int, NumericBoxed::Short, NumericContext::Array);
        }
    }
}
