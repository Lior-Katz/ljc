pub type Modified<T> = WithModifiers<T>;
pub type Modifiers = Vec<Modifier>;

#[derive(Debug)]
pub struct WithModifiers<T> {
    pub modifiers: Vec<Modifier>,
    pub item: T,
}

#[derive(Debug)]
pub enum Modifier {
    Public,
    Protected,
    Private,
    Abstract,
    Static,
    Final,
    Default,
}

pub trait Modifiable {
    fn with_modifiers(self, modifiers: Modifiers) -> WithModifiers<Self>
    where
        Self: Sized,
    {
        WithModifiers { modifiers, item: self }
    }
}

impl<T> Modifiable for T {}

impl<T> From<T> for Modified<T> {
    fn from(value: T) -> Self {
        value.with_modifiers(Modifiers::default())
    }
}
