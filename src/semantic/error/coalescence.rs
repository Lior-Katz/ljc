use crate::collections::{AtLeastOne, Multiple, TryCollect};
use crate::semantic::error::SemanticResult;

pub trait CoalesceIter<T, F>
where
    F: FnMut(T) -> SemanticResult,
    Self: IntoIterator<Item = T> + Sized,
{
    fn coalesce(self, mut op: F) -> SemanticResult {
        self.into_iter()
            .filter_map(|d| op(d).err())
            .flat_map(IntoIterator::into_iter)
            .try_collect()
            .map_or(Ok(()), Err)
    }
}

impl<'a, T, F> CoalesceIter<&'a T, F> for &'a Multiple<T> where F: FnMut(&'a T) -> SemanticResult {}

impl<'a, T, F> CoalesceIter<&'a T, F> for &'a AtLeastOne<T> where F: FnMut(&'a T) -> SemanticResult {}

pub trait CoalesceRes {
    fn coalesce(self, op: SemanticResult) -> SemanticResult;
}

impl CoalesceRes for SemanticResult {
    fn coalesce(self, new: SemanticResult) -> SemanticResult {
        match (self, new) {
            (Ok(_), result) => result,
            (Err(errors), Ok(_)) => Err(errors),
            (Err(mut errors), Err(new_errors)) => {
                errors.append(new_errors);
                Err(errors)
            }
        }
    }
}

pub trait Fold<T1> {
    fn fold<T2, F, R>(self, new: SemanticResult<T2>, fold: F) -> SemanticResult<R>
    where
        F: FnMut(T1, T2) -> SemanticResult<R>;
}

impl<T1> Fold<T1> for SemanticResult<T1> {
    fn fold<T2, F, R>(self, new: SemanticResult<T2>, mut fold: F) -> SemanticResult<R>
    where
        F: FnMut(T1, T2) -> SemanticResult<R>,
    {
        match (self, new) {
            (Ok(v1), Ok(v2)) => fold(v1, v2),
            (Ok(_), Err(e)) | (Err(e), Ok(_)) => Err(e),
            (Err(mut e1), Err(e2)) => {
                e1.append(e2);
                Err(e1)
            }
        }
    }
}
