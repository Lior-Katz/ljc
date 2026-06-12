use crate::ast::expressions::Expression;
use crate::ast::patterns::Pattern;
use crate::ast::statements::{BlockStatements, Statement};
use crate::collections::AtLeastOne;
use crate::file::Span;

pub type SwitchBlockMembers = Vec<SwitchBlockMember>;
pub type CaseConstant = Expression;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Switch {
    pub expression: Expression,
    pub block: SwitchBlockMembers,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum SwitchBlockMember {
    Rule {
        case: SwitchLabel,
        rule: SwitchRule,
    },
    LabeledStatements {
        labels: AtLeastOne<SwitchLabel>,
        statements: BlockStatements,
    },
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum SwitchLabel {
    Constants(AtLeastOne<CaseConstant>),
    Null {
        default: bool,
    },
    Default,
    Pattern {
        patterns: AtLeastOne<Pattern>,
        guard: Option<Expression>,
    },
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum SwitchRule {
    Expression(Expression),
    Block(BlockStatements),
    Throw(Statement),
}
