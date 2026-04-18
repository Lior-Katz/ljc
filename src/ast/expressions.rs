use crate::ast::identifiers::Identifier;
use crate::ast::types::Type;

pub type ExpressionList = Vec<Expression>;
pub type ArgumentList = Vec<Expression>;
pub type VariableInitializerList = Vec<VariableInitializer>;

#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(u64),
    LongLiteral(u64),
    BooleanLiteral(bool),
    CharLiteral(char),
    StringLiteral(String),
    NullLiteral,
    Name(Identifier),
    Assignment {
        lhs: LeftHandSide,
        rhs: Box<Expression>,
        op: AssignmentOp,
    },
    PostIncrement(Box<Expression>),
    PostDecrement(Box<Expression>),
    PreIncrement(Box<Expression>),
    PreDecrement(Box<Expression>),
    BitwiseComplement(Box<Expression>),
    LogicalNot(Box<Expression>),
    UnaryPlus(Box<Expression>),
    UnaryMinus(Box<Expression>),
    BinaryOp {
        left: Box<Expression>,
        right: Box<Expression>,
        op: BinOp,
    },
    ConditionalExpression {
        condition: Box<Expression>,
        if_true: Box<Expression>,
        if_false: Box<Expression>,
    },
    Type(Type),
    MemberAccess(MemberAccess),
    MethodCall(MethodCall),
    InstanceCreation {
        type_to_instantiate: Type,
        arguments: ArgumentList,
    },
    ArrayCreation {
        element_type: Type,
        array_creation_mode: ArrayCreationMode,
    },
}

#[derive(Debug)]
pub enum AssignmentOp {
    Add,
    Subtract,
    Identity,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    SignedRightShift,
    UnsignedRightShift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    SignedRightShift,
    UnsignedRightShift,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug)]
pub enum LeftHandSide {
    ExpressionName(Identifier),
    MemberAccess(MemberAccess),
}

#[derive(Debug)]
pub struct MemberAccess {
    pub target: Box<Expression>,
    pub name: Identifier,
}

#[derive(Debug)]
pub struct MethodCall {
    pub target: Box<Expression>,
    pub name: Identifier,
    pub arguments: ArgumentList,
}

#[derive(Debug)]
pub enum ArrayCreationMode {
    Sized {
        sized_dimensions: Vec<Expression>,
        unsized_dimensions: usize,
    },
    Initialized(VariableInitializerList),
}

#[derive(Debug)]
pub enum VariableInitializer {
    Expression(Expression),
    ArrayInitializer(VariableInitializerList),
}
