use crate::collections::{AtLeastOne, Multiple, TryCollect};
use crate::semantic::Diagnostic;

pub type SemanticResult<T = ()> = Result<T, AtLeastOne<Diagnostic>>;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

impl Into<AtLeastOne<Diagnostic>> for Diagnostic {
    fn into(self) -> AtLeastOne<Diagnostic> {
        AtLeastOne::new(self)
    }
}

pub trait Coalesce<T>
where
    Self: IntoIterator<Item = T>,
{
    fn coalesce<F>(&self, op: F) -> SemanticResult
    where
        F: Fn(&T) -> SemanticResult;
}

impl<T> Coalesce<T> for Multiple<T> {
    fn coalesce<F>(&self, op: F) -> SemanticResult
    where
        F: Fn(&T) -> SemanticResult,
    {
        self.into_iter()
            .filter_map(|d| op(d).err())
            .flat_map(IntoIterator::into_iter)
            .try_collect()
            .map_or(Ok(()), Err)
    }
}
