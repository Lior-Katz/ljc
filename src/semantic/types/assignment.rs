use crate::semantic::types::Type;

impl Type {
    pub fn is_assignable_from(&self, other: &Self) -> bool {
        // TODO: add assignment rules for conversions
        self == other
    }
}
