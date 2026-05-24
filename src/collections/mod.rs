pub mod non_empty_list;
pub use non_empty_list::{AtLeastOne, NonEmptyList};
pub mod bitflags;
pub(crate) use bitflags::bitflag_combination;
mod iter;
pub use iter::TryCollect;

pub type Multiple<T> = Vec<T>;
