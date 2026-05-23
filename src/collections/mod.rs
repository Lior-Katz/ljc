pub mod non_empty_list;
pub use non_empty_list::{AtLeastOne, NonEmptyList};
pub mod bitflags;
pub(crate) use bitflags::bitflag_combination;
pub type Multiple<T> = Vec<T>;
