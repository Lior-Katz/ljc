use crate::semantic::types::conversions::{boxing_conversion, unboxing_conversion};
use crate::semantic::types::{Numeric, Primitive, Type};

impl Type {
    pub fn is_assignable_from(&self, other: &Self) -> bool {
        // TODO: add assignment rules for references
        // TODO: add assignment rules for narrowing numeric conversions of constant expressions
        match (self, other) {
            (Type::Primitive(self_primitive), Type::Primitive(other_primitive)) => {
                self_primitive.is_assignable_from(other_primitive)
            }
            (Type::Primitive(self_primitive), Type::Boxed(self_boxed)) => {
                self_primitive.is_assignable_from(&unboxing_conversion(self_boxed.clone()))
            }
            (Type::Boxed(self_boxed), Type::Primitive(other_primitive)) => {
                *self_boxed == boxing_conversion(other_primitive.clone())
            }
            (Type::Boxed(self_boxed), Type::Boxed(other_boxed)) => self_boxed == other_boxed,
            _ => false,
        }
    }
}

impl Primitive {
    fn is_assignable_from(&self, other: &Self) -> bool {
        match (self, other) {
            (Primitive::Numeric(self_numeric), Primitive::Numeric(other_numeric)) => {
                self_numeric.is_assignable_from(other_numeric)
            }
            (Primitive::Boolean, Primitive::Boolean) => true,
            (Primitive::Boolean, Primitive::Numeric(_))
            | (Primitive::Numeric(_), Primitive::Boolean) => false,
        }
    }
}

impl Numeric {
    /// [§5.1.2](https://docs.oracle.com/javase/specs/jls/se26/html/jls-5.html#jls-5.1.2) - Widening primitive conversion
    ///
    /// 19 specific conversions on primitive types are called the widening primitive conversions:
    /// - byte to short, int, long, float, or double
    /// - short to int, long, float, or double
    /// - char to int, long, float, or double
    /// - int to long, float, or double
    /// - long to float or double
    /// - float to double
    fn is_assignable_from(&self, other: &Self) -> bool {
        self == other
            || match (self, other) {
                (
                    Numeric::Short
                    | Numeric::Int
                    | Numeric::Long
                    | Numeric::Float
                    | Numeric::Double,
                    Numeric::Byte,
                ) => true,
                (
                    Numeric::Int | Numeric::Long | Numeric::Float | Numeric::Double,
                    Numeric::Short,
                ) => true,
                (
                    Numeric::Int | Numeric::Long | Numeric::Float | Numeric::Double,
                    Numeric::Char,
                ) => true,
                (Numeric::Long | Numeric::Float | Numeric::Double, Numeric::Int) => true,
                (Numeric::Float | Numeric::Double, Numeric::Long) => true,
                (Numeric::Double, Numeric::Float) => true,
                _ => false,
            }
    }
}

#[cfg(test)]
mod tests {
    use crate::semantic::types::{Boxed, Numeric, NumericBoxed, Primitive, Type};

    #[test]
    fn identical_primitive_types_assignable() {
        assert!(Type::from(Numeric::Byte).is_assignable_from(&Numeric::Byte.into()));
        assert!(Type::from(Numeric::Short).is_assignable_from(&Numeric::Short.into()));
        assert!(Type::from(Numeric::Int).is_assignable_from(&Numeric::Int.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&Numeric::Long.into()));
        assert!(Type::from(Numeric::Char).is_assignable_from(&Numeric::Char.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&Numeric::Float.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Double.into()));
        assert!(Type::from(Primitive::Boolean).is_assignable_from(&Primitive::Boolean.into()));
    }

    #[test]
    fn identical_boxed_types_assignable() {
        assert!(Type::from(NumericBoxed::Byte).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(Type::from(NumericBoxed::Short).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(
            Type::from(NumericBoxed::Integer).is_assignable_from(&NumericBoxed::Integer.into())
        );
        assert!(Type::from(NumericBoxed::Long).is_assignable_from(&NumericBoxed::Long.into()));
        assert!(
            Type::from(NumericBoxed::Character).is_assignable_from(&NumericBoxed::Character.into())
        );
        assert!(Type::from(NumericBoxed::Float).is_assignable_from(&NumericBoxed::Float.into()));
        assert!(Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Double.into()));
        assert!(Type::from(Boxed::Boolean).is_assignable_from(&Boxed::Boolean.into()));
    }

    #[test]
    fn widening_primitive_conversion_assignable() {
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Byte.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Short.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Int.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Long.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Char.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&Numeric::Float.into()));

        assert!(Type::from(Numeric::Float).is_assignable_from(&Numeric::Byte.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&Numeric::Short.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&Numeric::Int.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&Numeric::Long.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&Numeric::Char.into()));

        assert!(Type::from(Numeric::Long).is_assignable_from(&Numeric::Byte.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&Numeric::Short.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&Numeric::Int.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&Numeric::Char.into()));

        assert!(Type::from(Numeric::Int).is_assignable_from(&Numeric::Byte.into()));
        assert!(Type::from(Numeric::Int).is_assignable_from(&Numeric::Short.into()));
        assert!(Type::from(Numeric::Int).is_assignable_from(&Numeric::Char.into()));

        assert!(Type::from(Numeric::Short).is_assignable_from(&Numeric::Byte.into()));
    }

    #[test]
    fn boxing_conversion_assignable() {
        assert!(Type::from(Boxed::Boolean).is_assignable_from(&Primitive::Boolean.into()));
        assert!(Type::from(NumericBoxed::Byte).is_assignable_from(&Numeric::Byte.into()));
        assert!(Type::from(NumericBoxed::Short).is_assignable_from(&Numeric::Short.into()));
        assert!(Type::from(NumericBoxed::Integer).is_assignable_from(&Numeric::Int.into()));
        assert!(Type::from(NumericBoxed::Long).is_assignable_from(&Numeric::Long.into()));
        assert!(Type::from(NumericBoxed::Character).is_assignable_from(&Numeric::Char.into()));
        assert!(Type::from(NumericBoxed::Float).is_assignable_from(&Numeric::Float.into()));
        assert!(Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Double.into()));
    }

    #[test]
    fn unboxing_conversion_assignable() {
        assert!(Type::from(Primitive::Boolean).is_assignable_from(&Boxed::Boolean.into()));
        assert!(Type::from(Numeric::Byte).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(Type::from(Numeric::Short).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(Type::from(Numeric::Int).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&NumericBoxed::Long.into()));
        assert!(Type::from(Numeric::Char).is_assignable_from(&NumericBoxed::Character.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&NumericBoxed::Float.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Double.into()));
    }

    #[test]
    fn unboxing_conversion_then_widening_primitive_conversion_assignable() {
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Long.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Character.into()));
        assert!(Type::from(Numeric::Double).is_assignable_from(&NumericBoxed::Float.into()));

        assert!(Type::from(Numeric::Float).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&NumericBoxed::Long.into()));
        assert!(Type::from(Numeric::Float).is_assignable_from(&NumericBoxed::Character.into()));

        assert!(Type::from(Numeric::Long).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(Type::from(Numeric::Long).is_assignable_from(&NumericBoxed::Character.into()));

        assert!(Type::from(Numeric::Int).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(Type::from(Numeric::Int).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(Type::from(Numeric::Int).is_assignable_from(&NumericBoxed::Character.into()));

        assert!(Type::from(Numeric::Short).is_assignable_from(&NumericBoxed::Byte.into()));
    }

    #[test]
    fn widening_numeric_conversion_boxed_to_boxed_unassignable() {
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Long.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Character.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&NumericBoxed::Float.into()));

        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&NumericBoxed::Long.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&NumericBoxed::Character.into()));

        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&NumericBoxed::Integer.into()));
        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&NumericBoxed::Character.into()));

        assert!(!Type::from(NumericBoxed::Integer).is_assignable_from(&NumericBoxed::Byte.into()));
        assert!(!Type::from(NumericBoxed::Integer).is_assignable_from(&NumericBoxed::Short.into()));
        assert!(!Type::from(NumericBoxed::Integer).is_assignable_from(&NumericBoxed::Character.into()));

        assert!(!Type::from(NumericBoxed::Short).is_assignable_from(&NumericBoxed::Byte.into()));
    }

    #[test]
    fn widening_numeric_conversion_primitive_to_boxed_unassignable() {
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Byte.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Short.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Int.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Long.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Char.into()));
        assert!(!Type::from(NumericBoxed::Double).is_assignable_from(&Numeric::Float.into()));

        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&Numeric::Byte.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&Numeric::Short.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&Numeric::Int.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&Numeric::Long.into()));
        assert!(!Type::from(NumericBoxed::Float).is_assignable_from(&Numeric::Char.into()));

        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&Numeric::Byte.into()));
        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&Numeric::Short.into()));
        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&Numeric::Int.into()));
        assert!(!Type::from(NumericBoxed::Long).is_assignable_from(&Numeric::Char.into()));

        assert!(!Type::from(NumericBoxed::Integer).is_assignable_from(&Numeric::Byte.into()));
        assert!(!Type::from(NumericBoxed::Integer).is_assignable_from(&Numeric::Short.into()));
        assert!(!Type::from(NumericBoxed::Integer).is_assignable_from(&Numeric::Char.into()));

        assert!(!Type::from(NumericBoxed::Short).is_assignable_from(&Numeric::Byte.into()));
    }

    #[test]
    fn null_to_primitive_unassignable() {
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Byte).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Short).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Int).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Long).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Char).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Float).is_assignable_from(&Type::Null));
        assert!(!Type::from(Numeric::Double).is_assignable_from(&Type::Null));
    }

    #[test]
    fn boolean_to_numeric_unassignable() {
        assert!(!Type::from(Numeric::Byte).is_assignable_from(&Primitive::Boolean.into()));
        assert!(!Type::from(Numeric::Short).is_assignable_from(&Primitive::Boolean.into()));
        assert!(!Type::from(Numeric::Int).is_assignable_from(&Primitive::Boolean.into()));
        assert!(!Type::from(Numeric::Long).is_assignable_from(&Primitive::Boolean.into()));
        assert!(!Type::from(Numeric::Char).is_assignable_from(&Primitive::Boolean.into()));
        assert!(!Type::from(Numeric::Float).is_assignable_from(&Primitive::Boolean.into()));
        assert!(!Type::from(Numeric::Double).is_assignable_from(&Primitive::Boolean.into()));
    }

    #[test]
    fn numeric_to_boolean_unassignable() {
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Byte.into()));
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Short.into()));
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Int.into()));
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Long.into()));
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Char.into()));
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Float.into()));
        assert!(!Type::from(Primitive::Boolean).is_assignable_from(&Numeric::Double.into()));
    }
}
