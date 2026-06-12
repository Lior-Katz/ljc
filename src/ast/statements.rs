use crate::ast::declarations::{VariableDeclaration, VariableDeclaratorId};
use crate::ast::expressions::{ArgumentList, Expression, ExpressionList};
use crate::ast::identifiers::Identifier;
use crate::ast::modifiers::Modified;
use crate::ast::switch::Switch;
use crate::ast::types::Type;
use crate::collections::{AtLeastOne, Multiple};
use crate::file::Span;

pub type BlockStatements = Multiple<Statement>;
pub type ForUpdate = ExpressionList;
pub type CatchClauseList = Multiple<CatchClause>;
pub type CatchType = AtLeastOne<Modified<Type>>;
pub type Resources = AtLeastOne<Resource>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Statement {
    EmptyStatement(Span),
    ExpressionStatement(Expression),
    Block(Block),
    VariableDeclaration(Modified<VariableDeclaration>),
    If {
        condition: Expression,
        if_true: Box<Statement>,
        if_false: Option<Box<Statement>>,
        span: Span,
    },
    While {
        condition: Expression,
        statement: Box<Statement>,
        span: Span,
    },
    For {
        initializer: ForInit,
        condition: Option<Expression>,
        update: ForUpdate,
        statement: Box<Statement>,
        span: Span,
    },
    ForEach {
        variable_declaration: Modified<VariableDeclaration>,
        iterable: Expression,
        statement: Box<Statement>,
        span: Span,
    },
    DoWhile {
        statement: Box<Statement>,
        condition: Expression,
        span: Span,
    },
    Labeled {
        label: Identifier,
        body: Box<Statement>,
    },
    Break {
        label: Option<Identifier>,
        span: Span,
    },
    Continue {
        label: Option<Identifier>,
        span: Span,
    },
    Assert {
        condition: Expression,
        detail_message: Option<Expression>,
        span: Span,
    },
    Return {
        value: Option<Expression>,
        span: Span,
    },
    Try {
        resources: Option<Resources>,
        try_block: BlockStatements,
        exception_handlers: CatchClauseList,
        finally_block: Option<BlockStatements>,
        span: Span,
    },
    Throw {
        value: Expression,
        span: Span,
    },
    Synchronized {
        lock: Expression,
        body: BlockStatements,
        span: Span,
    },
    Switch(Switch),
    Yield {
        value: Expression,
        span: Span,
    },
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Block {
    pub statements: BlockStatements,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ForInit {
    LocalVarDeclaration(Modified<VariableDeclaration>),
    Expressions(ExpressionList),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Resource {
    VariableDeclaration(Modified<VariableDeclaration>),
    VariableAccess(Expression),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CatchClause {
    pub catch_type: CatchType,
    pub var_id: VariableDeclaratorId,
    pub body: BlockStatements,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ConstructorInvocation {
    Alternate { arguments: ArgumentList },
}

impl Statement {
    pub fn span(&self) -> &Span {
        match self {
            Statement::EmptyStatement(span) => span,
            Statement::ExpressionStatement(e) => e.span(),
            Statement::Block(b) => &b.span,
            Statement::VariableDeclaration(v) => match v.modifiers.first() {
                None => v.item.span(),
                Some(m) => m.span(),
            },
            Statement::If { span, .. }
            | Statement::While { span, .. }
            | Statement::For { span, .. }
            | Statement::ForEach { span, .. }
            | Statement::DoWhile { span, .. }
            | Statement::Labeled {
                label: Identifier { span, .. }, ..
            }
            | Statement::Break { span, .. }
            | Statement::Continue { span, .. }
            | Statement::Assert { span, .. }
            | Statement::Return { span, .. }
            | Statement::Try { span, .. }
            | Statement::Throw { span, .. }
            | Statement::Synchronized { span, .. }
            | Statement::Switch(Switch { span, .. })
            | Statement::Yield { span, .. } => span,
        }
    }
}
