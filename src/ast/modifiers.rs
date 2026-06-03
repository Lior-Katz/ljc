use crate::ast::{Expression, Identifier, TypeName};
use crate::file::Span;

pub type Modified<T> = WithModifiers<T>;
pub type Modifiers = Vec<Modifier>;
pub type ElementValuePairList = Vec<ElementValuePair>;
pub type ElementValueList = Vec<ElementValue>;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct WithModifiers<T> {
    pub modifiers: Vec<Modifier>,
    pub item: T,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Modifier {
    Public(Span),
    Protected(Span),
    Private(Span),
    Abstract(Span),
    Static(Span),
    Final(Span),
    Default(Span),
    Sealed(Span),
    NonSealed(Span),
    Strictfp(Span),
    Native(Span),
    Transient(Span),
    Volatile(Span),
    Synchronized(Span),
    Annotation(Annotation),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Annotation {
    Marker {
        name: TypeName,
        span: Span,
    },
    SingleElement {
        name: TypeName,
        value: ElementValue,
        span: Span,
    },
    Normal {
        name: TypeName,
        values: ElementValuePairList,
        span: Span,
    },
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ElementValue {
    ConditionalExpression(Expression),
    ElementValueList(ElementValueList),
    Annotation(Box<Annotation>),
}

#[derive(Debug, Hash, Eq, PartialEq)]
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

impl Modifier {
    pub fn span(&self) -> &Span {
        match self {
            Modifier::Public(span)
            | Modifier::Protected(span)
            | Modifier::Private(span)
            | Modifier::Abstract(span)
            | Modifier::Static(span)
            | Modifier::Final(span)
            | Modifier::Default(span)
            | Modifier::Sealed(span)
            | Modifier::NonSealed(span)
            | Modifier::Strictfp(span)
            | Modifier::Native(span)
            | Modifier::Transient(span)
            | Modifier::Volatile(span)
            | Modifier::Synchronized(span) => span,
            Modifier::Annotation(a) => a.span(),
        }
    }
}

impl Annotation {
    pub fn span(&self) -> &Span {
        match self {
            Annotation::Marker { span, .. }
            | Annotation::SingleElement { span, .. }
            | Annotation::Normal { span, .. } => span,
        }
    }
}
