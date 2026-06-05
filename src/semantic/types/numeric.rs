use crate::semantic::types::{BooleanMaybeBoxed, Boxed, Primitive, Type};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Numeric {
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumericBoxed {
    Byte,
    Short,
    Character,
    Integer,
    Long,
    Float,
    Double,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NumericMaybeBoxed {
    Primitive(Numeric),
    Boxed(NumericBoxed),
}

pub enum Integral {
    Byte,
    Short,
    Int,
    Long,
    Char,
}

pub enum IntegralBoxed {
    Byte,
    Short,
    Integer,
    Long,
    Character,
}

pub enum IntegralMaybeBoxed {
    Primitive(Integral),
    Boxed(IntegralBoxed),
}

impl Numeric {
    pub fn is_integral(&self) -> bool {
        self.as_integral().is_some()
    }

    pub fn as_integral(&self) -> Option<Integral> {
        match self {
            Numeric::Byte => Some(Integral::Byte),
            Numeric::Short => Some(Integral::Short),
            Numeric::Int => Some(Integral::Int),
            Numeric::Long => Some(Integral::Long),
            Numeric::Char => Some(Integral::Char),
            Numeric::Float | Numeric::Double => None,
        }
    }
}

impl NumericBoxed {
    pub fn is_integral(&self) -> bool {
        self.as_integral().is_some()
    }

    pub fn as_integral(&self) -> Option<IntegralBoxed> {
        match self {
            NumericBoxed::Byte => Some(IntegralBoxed::Byte),
            NumericBoxed::Short => Some(IntegralBoxed::Short),
            NumericBoxed::Integer => Some(IntegralBoxed::Integer),
            NumericBoxed::Long => Some(IntegralBoxed::Long),
            NumericBoxed::Character => Some(IntegralBoxed::Character),
            NumericBoxed::Float | NumericBoxed::Double => None,
        }
    }
}

impl Type {
    pub fn is_convertible_to_numeric_type(&self) -> bool {
        self.as_numeric_maybe_boxed().is_some()
    }

    /// [§5.1.8](https://docs.oracle.com/javase/specs/jls/se26/html/jls-5.html#jls-5.1.8) - A type is said to be convertible to a numeric type if it is a numeric type (§4.2), or it is a reference type that may be converted to a numeric type by unboxing conversion.
    pub fn as_numeric_maybe_boxed(&self) -> Option<NumericMaybeBoxed> {
        self.as_numeric()
            .map(Into::into)
            .or(self.as_numeric_boxed().map(Into::into))
    }

    /// [§4.2](https://docs.oracle.com/javase/specs/jls/se26/html/jls-4.html#jls-4.2) - The numeric types are the integral types and the floating-point types.
    fn as_numeric(&self) -> Option<Numeric> {
        self.as_primitive()?.as_numeric()
    }

    fn as_numeric_boxed(&self) -> Option<NumericBoxed> {
        self.as_boxed()?.as_numeric()
    }

    pub fn is_convertible_to_integral_type(&self) -> bool {
        self.as_numeric_maybe_boxed().is_some()
    }

    pub fn as_integral_maybe_boxed(&self) -> Option<IntegralMaybeBoxed> {
        self.as_integral()
            .map(Into::into)
            .or(self.as_integral_boxed().map(Into::into))
    }

    fn as_integral(&self) -> Option<Integral> {
        self.as_numeric()?.as_integral()
    }

    fn as_integral_boxed(&self) -> Option<IntegralBoxed> {
        self.as_numeric_boxed()?.as_integral()
    }

    pub fn is_primitive_or_boxed_boolean(&self) -> bool {
        self.as_boolean_maybe_boxed().is_some()
    }

    pub fn as_boolean_maybe_boxed(&self) -> Option<BooleanMaybeBoxed> {
        matches!(self.as_primitive(), Some(Primitive::Boolean))
            .then_some(BooleanMaybeBoxed::Primitive)
            .or(matches!(self.as_boxed(), Some(Boxed::Boolean)).then_some(BooleanMaybeBoxed::Boxed))
    }
}

impl From<Numeric> for NumericMaybeBoxed {
    fn from(value: Numeric) -> Self {
        NumericMaybeBoxed::Primitive(value)
    }
}

impl From<NumericBoxed> for NumericMaybeBoxed {
    fn from(value: NumericBoxed) -> Self {
        NumericMaybeBoxed::Boxed(value)
    }
}

impl From<Integral> for IntegralMaybeBoxed {
    fn from(value: Integral) -> Self {
        IntegralMaybeBoxed::Primitive(value)
    }
}

impl From<IntegralBoxed> for IntegralMaybeBoxed {
    fn from(value: IntegralBoxed) -> Self {
        IntegralMaybeBoxed::Boxed(value)
    }
}

impl From<Integral> for Numeric {
    fn from(value: Integral) -> Self {
        match value {
            Integral::Byte => Numeric::Byte,
            Integral::Short => Numeric::Short,
            Integral::Int => Numeric::Int,
            Integral::Long => Numeric::Long,
            Integral::Char => Numeric::Char,
        }
    }
}

impl From<IntegralBoxed> for NumericBoxed {
    fn from(value: IntegralBoxed) -> Self {
        match value {
            IntegralBoxed::Byte => NumericBoxed::Byte,
            IntegralBoxed::Short => NumericBoxed::Short,
            IntegralBoxed::Integer => NumericBoxed::Integer,
            IntegralBoxed::Long => NumericBoxed::Long,
            IntegralBoxed::Character => NumericBoxed::Character,
        }
    }
}

impl From<IntegralMaybeBoxed> for NumericMaybeBoxed {
    fn from(value: IntegralMaybeBoxed) -> Self {
        match value {
            IntegralMaybeBoxed::Primitive(integral) => Numeric::from(integral).into(),
            IntegralMaybeBoxed::Boxed(integral_boxed) => NumericBoxed::from(integral_boxed).into(),
        }
    }
}

impl From<NumericMaybeBoxed> for Type {
    fn from(value: NumericMaybeBoxed) -> Self {
        match value {
            NumericMaybeBoxed::Primitive(numeric) => numeric.into(),
            NumericMaybeBoxed::Boxed(numeric_boxed) => numeric_boxed.into(),
        }
    }
}

impl From<Integral> for Type {
    fn from(value: Integral) -> Self {
        Self::from(Numeric::from(value))
    }
}

impl From<IntegralBoxed> for Type {
    fn from(value: IntegralBoxed) -> Self {
        Self::from(NumericBoxed::from(value))
    }
}

impl From<IntegralMaybeBoxed> for Type {
    fn from(value: IntegralMaybeBoxed) -> Self {
        match value {
            IntegralMaybeBoxed::Primitive(integral) => integral.into(),
            IntegralMaybeBoxed::Boxed(integral_boxed) => integral_boxed.into(),
        }
    }
}
