mod numeric;
pub use numeric::Numeric;

#[derive(Eq, PartialEq)]
pub enum Type {
    Numeric(Numeric),
    Boolean,
    Null,
}

impl From<Numeric> for Type {
    fn from(value: Numeric) -> Self {
        Self::Numeric(value)
    }
}
