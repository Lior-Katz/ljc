pub type Program = CompilationUnit;
pub type Identifier = String;
pub type TypeIdentifier = Identifier;
type BlockStatements = Vec<Statement>;
pub type VariableDeclaratorList = Vec<VariableDeclarator>;
pub type FormalParameterList = Vec<Modified<FormalParameter>>;
pub type ArgumentList = Vec<Expression>;
pub type Modifiers = Vec<Modifier>;
pub type Modified<T> = WithModifiers<T>;
pub type MethodResult = Expression;

#[derive(Debug)]
pub enum CompilationUnit {
    Ordinary(Vec<TopLevelClassOrInterfaceDeclaration>),
}

#[derive(Debug)]
pub enum TopLevelClassOrInterfaceDeclaration {
    ClassDeclaration(Modified<ClassDeclaration>),
}

#[derive(Debug)]
pub struct WithModifiers<T> {
    pub modifiers: Vec<Modifier>,
    pub item: T,
}

#[derive(Debug)]
pub enum Modifier {
    Public,
    Protected,
    Private,
    Abstract,
    Static,
    Final,
}

pub trait Modifiable {
    fn with_modifiers(self, modifiers: Modifiers) -> WithModifiers<Self>
    where
        Self: Sized,
    {
        WithModifiers { modifiers, item: self }
    }
}

impl<T> Modifiable for T {}

#[derive(Debug)]
pub enum ClassDeclaration {
    NormalClassDeclaration(NormalClassDeclaration),
}

#[derive(Debug)]
pub struct NormalClassDeclaration {
    pub identifier: TypeIdentifier,
    pub body: Vec<ClassBodyDeclaration>,
}

#[derive(Debug)]
pub enum ClassBodyDeclaration {
    ClassMemberDeclaration(Modified<ClassMemberDeclaration>),
}

#[derive(Debug)]
pub enum ClassMemberDeclaration {
    MethodDeclaration(MethodDeclaration),
    NestedClassDeclaration(ClassDeclaration),
    FieldDeclaration {
        variable_type: Expression,
        declarations: VariableDeclaratorList,
    },
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub result: MethodResult,
    pub identifier: Identifier,
    pub parameters: FormalParameterList,
    pub body: MethodBody,
}

#[derive(Debug)]
pub enum FormalParameter {
    NormalFormalParameter(Type, VariableDeclaratorId),
    VariableArityParameter(Type, Identifier),
}

#[derive(Debug)]
pub enum MethodBody {
    Semicolon,
    Block(BlockStatements),
}

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
    ExpressionStatement(Expression),
    Block(BlockStatements),
    VariableDeclaration(Modified<VariableDeclaration>),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub variable_type: Expression,
    pub declarators: VariableDeclaratorList,
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub name: VariableDeclaratorId,
    pub initializer: Option<VariableInitializer>,
}

#[derive(Debug)]
pub enum VariableDeclaratorId {
    Named(Identifier),
    Unnamed,
}

#[derive(Debug)]
pub enum VariableInitializer {
    Expression(Expression),
}

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
pub enum Type {
    // primitive types
    Byte,
    Short,
    Int,
    Long,
    Char,
    Float,
    Double,
    Boolean,
    Void,
}
