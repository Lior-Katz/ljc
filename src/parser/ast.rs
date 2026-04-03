pub type Program = CompilationUnit;
pub type Identifier = String;
pub type TypeIdentifier = Identifier;
type BlockStatements = Vec<Statement>;

#[derive(Debug)]
pub enum CompilationUnit {
    Ordinary(Vec<TopLevelClassOrInterfaceDeclaration>),
}

#[derive(Debug)]
pub enum TopLevelClassOrInterfaceDeclaration {
    ClassDeclaration(ClassDeclaration),
}

#[derive(Debug)]
pub enum ClassDeclaration {
    NormalClassDeclaration(NormalClassDeclaration),
}

#[derive(Debug)]
pub struct NormalClassDeclaration {
    pub modifiers: Vec<ClassModifier>,
    pub identifier: TypeIdentifier,
    pub body: ClassBody,
}

#[derive(Debug)]
pub enum ClassModifier {
    Public,
    Protected,
    Private,
}
#[derive(Debug)]
pub struct ClassBody {
    pub class_body_declarations: Vec<ClassBodyDeclaration>,
}

#[derive(Debug)]
pub enum ClassBodyDeclaration {
    ClassMemberDeclaration(ClassMemberDeclaration),
}

#[derive(Debug)]
pub enum ClassMemberDeclaration {
    MethodDeclaration(MethodDeclaration),
}

#[derive(Debug)]
pub struct MethodDeclaration {
    pub modifiers: Vec<MethodModifiers>,
    pub result: MethodResult,
    pub identifier: Identifier,
    pub parameters: Vec<FormalParameter>,
    pub body: MethodBody,
}

#[derive(Debug)]
pub enum MethodModifiers {
    Public,
    Protected,
    Private,
}

#[derive(Debug)]
pub enum MethodResult {
    Void,
    Type(Type),
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
    },
    PostIncrement(Box<Expression>),
    PostDecrement(Box<Expression>),
    BitwiseComplement(Box<Expression>),
    LogicalNot(Box<Expression>),
    UnaryPlus(Box<Expression>),
    UnaryMinus(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum LeftHandSide {
    ExpressionName(Identifier),
}

#[derive(Debug)]
pub struct VariableDeclaratorId {
    pub identifier: Identifier,
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
}
