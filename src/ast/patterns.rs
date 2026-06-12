use crate::ast::declarations::VariableDeclaration;
use crate::ast::modifiers::Modified;
use crate::ast::types::Type;
use crate::collections::Multiple;

pub type ComponentPatternList = Multiple<ComponentPattern>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Pattern {
    Type(Modified<VariableDeclaration>),
    Record {
        reference_type: Type,
        components: ComponentPatternList,
    },
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ComponentPattern {
    Pattern(Pattern),
    MatchAll,
}
