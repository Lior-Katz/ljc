use crate::ast::{Expression, Identifier, TypeName};

pub type Modified<T> = WithModifiers<T>;
pub type Modifiers = Vec<Modifier>;
pub type ElementValuePairList = Vec<ElementValuePair>;
pub type ElementValueList = Vec<ElementValue>;

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
    Sealed,
    NonSealed,
    Annotation(Annotation),
}

#[derive(Debug)]
pub enum Annotation {
    Marker(TypeName),
    SingleElement {
        name: TypeName,
        value: ElementValue,
    },
    Normal {
        name: TypeName,
        values: ElementValuePairList,
    },
}

#[derive(Debug)]
pub enum ElementValue {
    ConditionalExpression(Expression),
    ElementValueList(ElementValueList),
    Annotation(Box<Annotation>),
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub(crate) name: Identifier,
    pub(crate) value: ElementValue,
}

impl Into<Modifier> for Annotation {
    fn into(self) -> Modifier {
        Modifier::Annotation(self)
    }
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
