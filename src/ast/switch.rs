use crate::ast::{BlockStatements, Expression, Multiple};
use crate::ast::patterns::Pattern;

pub type SwitchBlockMembers = Vec<SwitchBlockMember>;
pub type CaseConstant = Expression;

#[derive(Debug)]
pub struct Switch {
    pub expression: Expression,
    pub block: SwitchBlockMembers,
}

#[derive(Debug)]
pub enum SwitchBlockMember {
    LabeledStatements {
        labels: Multiple<SwitchLabel>,
        statements: BlockStatements,
    },
}

#[derive(Debug)]
pub enum SwitchLabel {
    Constants(Multiple<CaseConstant>),
    Null {
        default: bool,
    },
    Default,
    Pattern(Multiple<Pattern>),
}
