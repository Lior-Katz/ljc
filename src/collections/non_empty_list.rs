use crate::collections::iter::TryFromIterator;
use std::iter::{Chain, Once};
use std::slice::Iter;

pub type AtLeastOne<T> = NonEmptyList<T>;

#[derive(Debug, Clone, Copy)]
pub struct EmptyVecError;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct NonEmptyList<T> {
    head: T,
    rest: Vec<T>,
}

impl<T> NonEmptyList<T> {
    pub fn new(element: T) -> Self {
        Self {
            head: element,
            rest: Vec::new(),
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Result<Self, EmptyVecError> {
        let mut iter = vec.into_iter();
        let head = iter.next().ok_or(EmptyVecError)?;
        Ok(Self { head, rest: iter.collect() })
    }

    pub fn to_vec(mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        vec.push(self.head);
        vec.append(&mut self.rest);
        vec
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub fn push(&mut self, element: T) {
        self.rest.push(element);
    }

    pub fn append_vec(&mut self, mut other: Vec<T>) {
        self.rest.append(&mut other);
    }

    pub fn append(&mut self, mut other: Self) {
        self.rest.push(other.head);
        self.rest.append(&mut other.rest);
    }

    pub fn split_last(mut self) -> (Vec<T>, T) {
        if let Some(last) = self.rest.pop() {
            (self.to_vec(), last)
        } else {
            (self.rest, self.head) // self.rest is already empty
        }
    }

    pub fn iter(&self) -> Chain<Once<&T>, Iter<'_, T>> {
        std::iter::once(&self.head).chain((&self.rest).into_iter())
    }
}

impl<T> IntoIterator for NonEmptyList<T> {
    type Item = T;

    type IntoIter = Chain<Once<T>, std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.head).chain(self.rest.into_iter())
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyList<T> {
    type Item = &'a T;

    type IntoIter = Chain<Once<&'a T>, Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, I> TryFromIterator<I, T> for NonEmptyList<T>
where
    I: IntoIterator<Item = T>,
{
    type Error = EmptyVecError;

    fn try_from_iter(iter: I) -> Result<Self, Self::Error> {
        let mut iter = iter.into_iter();
        let head = iter.next().ok_or(EmptyVecError)?;
        Ok(Self { head, rest: iter.collect() })
    }
}

impl<T> Into<Vec<T>> for NonEmptyList<T> {
    fn into(self) -> Vec<T> {
        self.to_vec()
    }
}
