pub trait TryFromIterator<IterLike, A>: Sized
where
    IterLike: IntoIterator<Item = A>,
{
    type Error;

    fn try_from_iter(iter: IterLike) -> Result<Self, Self::Error>;
}

pub trait TryCollect<Collection> {
    type Error;

    fn try_collect(self) -> Result<Collection, Self::Error>;
}

impl<IterLike, A, Collection> TryCollect<Collection> for IterLike
where
    IterLike: IntoIterator<Item = A>,
    Collection: TryFromIterator<IterLike, A>,
{
    type Error = <Collection as TryFromIterator<IterLike, A>>::Error;

    fn try_collect(self) -> Result<Collection, Self::Error> {
        Collection::try_from_iter(self)
    }
}
