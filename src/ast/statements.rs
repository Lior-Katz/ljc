use crate::ast::declarations::{VariableDeclaration, VariableDeclaratorId};
use crate::ast::expressions::{ArgumentList, Expression, ExpressionList};
use crate::ast::identifiers::Identifier;
use crate::ast::modifiers::Modified;
use crate::ast::switch::Switch;
use crate::ast::types::Type;

pub type BlockStatements = Vec<Statement>;
pub type ForUpdate = ExpressionList;
pub type CatchClauseList = Vec<CatchClause>;
pub type CatchType = Vec<Modified<Type>>;
pub type Resources = Vec<Resource>;

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
    ExpressionStatement(Expression),
    Block(BlockStatements),
    VariableDeclaration(Modified<VariableDeclaration>),
    If {
        condition: Expression,
        if_true: Box<Statement>,
        if_false: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        statement: Box<Statement>,
    },
    For {
        initializer: ForInit,
        condition: Option<Expression>,
        update: ForUpdate,
        statement: Box<Statement>,
    },
    ForEach {
        variable_declaration: Modified<VariableDeclaration>,
        iterable: Expression,
        statement: Box<Statement>,
    },
    DoWhile {
        statement: Box<Statement>,
        condition: Expression,
    },
    Labeled {
        label: Identifier,
        body: Box<Statement>,
    },
    Break(Option<Identifier>),
    Continue(Option<Identifier>),
    Assert {
        condition: Expression,
        detail_message: Option<Expression>,
    },
    Return(Option<Expression>),
    Try {
        resource: Resources,
        try_block: BlockStatements,
        exception_handlers: CatchClauseList,
        finally_block: Option<BlockStatements>,
    },
    Throw(Expression),
    Synchronized {
        lock: Expression,
        body: BlockStatements,
    },
    Switch(Switch),
}

#[derive(Debug)]
pub enum ForInit {
    LocalVarDeclaration(Modified<VariableDeclaration>),
    Expressions(ExpressionList),
}

#[derive(Debug)]
pub enum Resource {
    VariableDeclaration(Modified<VariableDeclaration>),
    VariableAccess(Expression),
}

#[derive(Debug)]
pub struct CatchClause {
    pub catch_type: CatchType,
    pub var_id: VariableDeclaratorId,
    pub body: BlockStatements,
}

#[derive(Debug)]
pub enum ConstructorInvocation {
    Alternate { arguments: ArgumentList },
}
