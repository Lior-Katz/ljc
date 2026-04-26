use crate::ast::Multiple;
use crate::ast::declarations::VariableDeclaration;
use crate::ast::modifiers::Modified;
use crate::ast::types::Type;

pub type ComponentPatternList = Multiple<ComponentPattern>;

#[derive(Debug)]
pub enum Pattern {
    Type(Modified<VariableDeclaration>),
    Record {
        reference_type: Type,
        components: ComponentPatternList,
    },
}

#[derive(Debug)]
pub enum ComponentPattern {
    Pattern(Pattern),
    MatchAll,
}
