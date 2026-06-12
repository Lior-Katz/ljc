use crate::ast::TypeOrVoid;
use crate::ast::identifiers::Identifier;
use crate::ast::switch::Switch;
use crate::ast::types::Type;
use crate::file::Span;

pub type ExpressionList = Vec<Expression>;
pub type ArgumentList = Vec<Expression>;
pub type VariableInitializerList = Vec<VariableInitializer>;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Expression {
    IntegerLiteral {
        value: u64,
        span: Span,
    },
    LongLiteral {
        value: u64,
        span: Span,
    },
    BooleanLiteral {
        value: bool,
        span: Span,
    },
    CharLiteral {
        value: char,
        span: Span,
    },
    StringLiteral {
        value: String,
        span: Span,
    },
    NullLiteral(Span),
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
    ArrayAccess(ArrayAccess),
    Switch(Box<Switch>),
    This(Span),
    QualifiedThis(Type),
    ClassLiteral(TypeOrVoid),
    MethodReference {
        target: Box<ExpressionOrType>,
        method: MethodReferenceType,
    },
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExpressionOrType {
    Expression(Expression),
    Type(TypeOrVoid),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum LeftHandSide {
    ExpressionName(Identifier),
    MemberAccess(MemberAccess),
    ArrayAccess(ArrayAccess),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct MemberAccess {
    pub target: Box<Expression>,
    pub name: Identifier,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct MethodCall {
    pub target: Option<Box<Expression>>,
    pub name: Identifier,
    pub arguments: ArgumentList,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ArrayCreationMode {
    Sized {
        sized_dimensions: Vec<Expression>,
        unsized_dimensions: usize,
    },
    Initialized(ArrayInitializer),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ArrayAccess {
    pub target: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum VariableInitializer {
    Expression(Expression),
    ArrayInitializer(ArrayInitializer),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ArrayInitializer {
    pub initializer: VariableInitializerList,
    pub span: Span,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum MethodReferenceType {
    Constructor,
    Named(Identifier),
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Self::IntegerLiteral { span, .. }
            | Self::LongLiteral { span, .. }
            | Self::BooleanLiteral { span, .. }
            | Self::CharLiteral { span, .. }
            | Self::StringLiteral { span, .. }
            | Self::NullLiteral(span)
            | Self::Name(Identifier { span, .. })
            | Self::This(span) => span,

            Self::PostIncrement(expression)
            | Self::PostDecrement(expression)
            | Self::PreIncrement(expression)
            | Self::PreDecrement(expression)
            | Self::BitwiseComplement(expression)
            | Self::LogicalNot(expression)
            | Self::UnaryPlus(expression)
            | Self::UnaryMinus(expression)
            | Self::BinaryOp { left: expression, .. }
            | Self::ConditionalExpression { condition: expression, .. } => expression.span(),

            Self::MethodReference { target, .. } => target.span(),

            Self::QualifiedThis(t) => t.span(),
            Self::ClassLiteral(t) => t.span(),
            Self::InstanceCreation { type_to_instantiate: t, .. }
            | Self::ArrayCreation { element_type: t, .. } => t.span(),

            Self::Assignment { lhs, .. } => lhs.span(),
            Self::MethodCall(v) => v.span(),
            Self::MemberAccess(v) => v.span(),
            Self::ArrayAccess(v) => v.span(),
            Self::Switch(v) => &v.span,
        }
    }
}

impl ExpressionOrType {
    pub fn span(&self) -> &Span {
        match self {
            ExpressionOrType::Expression(e) => e.span(),
            ExpressionOrType::Type(ty) => ty.span(),
        }
    }
}

impl LeftHandSide {
    pub fn span(&self) -> &Span {
        match self {
            Self::ExpressionName(id) => &id.span,
            Self::MemberAccess(member_access) => member_access.span(),
            Self::ArrayAccess(array_access) => array_access.span(),
        }
    }
}

impl MethodCall {
    fn span(&self) -> &Span {
        match &self.target {
            Some(e) => e.span(),
            None => &self.name.span,
        }
    }
}

impl MemberAccess {
    pub fn span(&self) -> &Span {
        self.target.span()
    }
}

impl ArrayAccess {
    fn span(&self) -> &Span {
        self.target.span()
    }
}
