use crate::ast::{CompilationUnit, TypeDeclaration};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

mod compilation_unit;
mod type_declaration;

pub use crate::semantic::ast_tags::compilation_unit::CompilationUnitAttributes;
pub use crate::semantic::ast_tags::type_declaration::TypeDeclarationAttributes;

macro_rules! attributes {
    ($($node:ident => $attr:ty),+ $(,)?) => {
        #[derive(Debug, Hash, Eq, PartialEq)]
        pub enum Key<'a> {
        $(
            $node(&'a $node),
        )+
        }

        pub enum NodeAttribute {
        $(
            $node($attr),
        )+
        }

    $(
        impl<'a> HasAttributes<'a, $attr> for $node {
            fn wrap_attribute(attrs: $attr) -> NodeAttribute {
                NodeAttribute::$node(attrs)
            }

            fn unwrap_attribute(attributes: &'a NodeAttribute) -> Option<&'a $attr> {
                match attributes {
                    NodeAttribute::$node(a) => Some(a),
                    _ => None,
                }
            }

            fn unwrap_attribute_mut(
                attributes: &'a mut NodeAttribute,
            ) -> Option<&'a mut $attr> {
                match attributes {
                    NodeAttribute::$node(a) => Some(a),
                    _ => None,
                }
            }
        }

        impl<'a> From<&'a $node> for Key<'a> {
            fn from(value: &'a $node) -> Self {
                Key::$node(value)
            }
        }
    )+
    };
}

attributes!(
    CompilationUnit => CompilationUnitAttributes,
    TypeDeclaration => TypeDeclarationAttributes,
);

pub struct Attributes<'a>(HashMap<Key<'a>, NodeAttribute>);

impl<'a> Attributes<'a> {
    pub fn new() -> Self {
        Attributes(HashMap::new())
    }

    pub fn insert<Node, Attrs>(&mut self, node: &'a Node, attrs: Attrs)
    where
        Node: Hash + HasAttributes<'a, Attrs>,
        Key<'a>: From<&'a Node>,
    {
        self.0.insert(Key::from(node), Node::wrap_attribute(attrs));
    }

    #[expect(dead_code)]
    pub fn get<Node, Attrs>(&'a self, node: &'a Node) -> Option<&'a Attrs>
    where
        Node: Hash + HasAttributes<'a, Attrs>,
        Key<'a>: From<&'a Node>,
    {
        self.0
            .get(&Key::from(node))
            .and_then(Node::unwrap_attribute)
    }

    #[expect(dead_code)]
    pub fn get_mut<Node, Attrs>(&'a mut self, node: &'a Node) -> Option<&'a mut Attrs>
    where
        Node: Hash + HasAttributes<'a, Attrs>,
        Key<'a>: From<&'a Node>,
    {
        self.0
            .get_mut(&Key::from(node))
            .and_then(Node::unwrap_attribute_mut)
    }
}

pub trait HasAttributes<'a, Attrs> {
    fn wrap_attribute(attrs: Attrs) -> NodeAttribute;
    fn unwrap_attribute(attributes: &'a NodeAttribute) -> Option<&'a Attrs>;
    fn unwrap_attribute_mut(attributes: &'a mut NodeAttribute) -> Option<&'a mut Attrs>;
}
