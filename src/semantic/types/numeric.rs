use crate::semantic::types::Type;

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum Numeric {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
}

impl Numeric {
    pub fn is_integral(&self) -> bool {
        match self {
            Numeric::Byte | Numeric::Short | Numeric::Int | Numeric::Long | Numeric::Char => true,
            Numeric::Float | Numeric::Double => false,
        }
    }
}

impl Type {
    /// [§5.1.8](https://docs.oracle.com/javase/specs/jls/se26/html/jls-5.html#jls-5.1.8) - A type is said to be convertible to a numeric type if it is a numeric type (§4.2), or it is a reference type that may be converted to a numeric type by unboxing conversion.
    pub fn is_convertible_to_numeric_type(&self) -> bool {
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

    pub fn is_convertible_to_integral_type(&self) -> bool {
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
    pub fn is_primitive_or_boxed_boolean(&self) -> bool {
        matches!(self, Self::Boolean)
        // TODO: check for boxed Boolean as well
    }
}
