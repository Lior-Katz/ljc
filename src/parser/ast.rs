pub type Program = CompilationUnit;
pub type Identifier = String;
pub type TypeIdentifier = Identifier;
type BlockStatements = Vec<Statement>;
pub type VariableDeclaratorList = Vec<VariableDeclarator>;
pub type VariableInitializerList = Vec<VariableInitializer>;
pub type FormalParameterList = Vec<Modified<FormalParameter>>;
pub type ArgumentList = Vec<Expression>;
pub type Modifiers = Vec<Modifier>;
pub type Modified<T> = WithModifiers<T>;
pub type MethodResult = Expression;
pub type ExpressionList = Vec<Expression>;
pub type ForUpdate = ExpressionList;
pub type CatchClauseList = Vec<CatchClause>;
pub type ClassType = Vec<ClassTypePart>;
pub type CatchType = Vec<Type>;
pub type Resources = Vec<Resource>;

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
    ConstructorDeclaration {
        name: Identifier, // this is just for validating that the name matches the class
        parameters: FormalParameterList,
        body: ConstructorBody,
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
pub struct ConstructorBody {
    pub prologue: Option<BlockStatements>,
    pub constructor_invocation: Option<ConstructorInvocation>,
    pub epilogue: BlockStatements,
}

#[derive(Debug)]
pub enum ConstructorInvocation {
    Alternate { arguments: ArgumentList },
}

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
    ExpressionStatement(Expression),
    Block(BlockStatements),
    VariableDeclaration(Modified<VariableDeclaration>),
    IfStatement {
        condition: Expression,
        if_true: Box<Statement>,
        if_false: Option<Box<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        statement: Box<Statement>,
    },
    ForStatement {
        initializer: ForInit,
        condition: Option<Expression>,
        update: ForUpdate,
        statement: Box<Statement>,
    },
    ForEachStatement {
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
}

#[derive(Debug)]
pub enum ForInit {
    LocalVarDeclaration(Modified<VariableDeclaration>),
    Expressions(ExpressionList),
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
    ArrayInitializer(VariableInitializerList)
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
    }
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

    // reference types
    ClassType(ClassType),
    ArrayType(ArrayType),
}

#[derive(Debug)]
pub struct ClassTypePart {
    pub identifier: Identifier,
}

#[derive(Debug)]
pub struct ArrayType {
    pub element_type: Box<Type>,
}

#[derive(Debug)]
pub enum ArrayCreationMode {
    Sized {
        sized_dimensions: Vec<Expression>,
        unsized_dimensions: usize,
    },
    Initialized(VariableInitializerList),
}
