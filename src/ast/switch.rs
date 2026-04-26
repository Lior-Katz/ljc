use crate::ast::Multiple;
use crate::ast::expressions::Expression;
use crate::ast::patterns::Pattern;
use crate::ast::statements::{BlockStatements, Statement};

pub type SwitchBlockMembers = Vec<SwitchBlockMember>;
pub type CaseConstant = Expression;

#[derive(Debug)]
pub struct Switch {
    pub expression: Expression,
    pub block: SwitchBlockMembers,
}

#[derive(Debug)]
pub enum SwitchBlockMember {
    Rule {
        case: SwitchLabel,
        rule: SwitchRule,
    },
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
    Pattern {
        patterns: Multiple<Pattern>,
        guard: Option<Expression>,
    },
}

#[derive(Debug)]
pub enum SwitchRule {
    Expression(Expression),
    Block(BlockStatements),
    Throw(Statement),
}
