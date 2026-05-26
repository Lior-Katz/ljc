use crate::collections::AtLeastOne;
use crate::semantic::Diagnostic;
use std::fmt::Display;

pub type SemanticResult<T = ()> = Result<T, AtLeastOne<Diagnostic>>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0} not yet supported")]
    Unimplemented(#[from] UnimplementedFeature),

    #[error(transparent)]
    TypeMismatch(#[from] TypeMismatch),
}

#[derive(thiserror::Error, Debug)]
pub enum TypeMismatch {
    #[error("Expected a variable, but found value")]
    NeedVariableFoundValue,

    #[error("Expression of type 'void' is not allowed here")]
    VoidExpression,

    #[error(
        "Expected a numeric operand.\n\
             Hint: an operand is numeric if it is of a numeric type (i.e. byte, short, int, long, char, float, or double),\n\
             or if it is a reference type and can be unboxed to a numeric type (i.e. Byte, Short, Integer, Long, Character, Float, or Double)."
    )]
    NonNumericOperand,

    #[error("Expected an operand of type boolean or Boolean")]
    NonBooleanOperand,
}

trait SubError: Into<Error> + Display {}

impl SubError for TypeMismatch {}

impl Into<AtLeastOne<Diagnostic>> for Diagnostic {
    fn into(self) -> AtLeastOne<Diagnostic> {
        AtLeastOne::new(self)
    }
}

impl<T> From<crate::error::Diagnostic<T>> for AtLeastOne<Diagnostic>
where
    T: SubError,
{
    fn from(value: crate::error::Diagnostic<T>) -> Self {
        Diagnostic::from(value).into()
    }
}

impl<T> From<crate::error::Diagnostic<T>> for Diagnostic
where
    T: SubError,
{
    fn from(value: crate::error::Diagnostic<T>) -> Self {
        Diagnostic {
            span: value.span,
            message: value.message.into(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UnimplementedFeature {
    #[error("String literals")]
    StringLiteral,
    #[error("Name expressions")]
    NameExpression,
    #[error("Assignment expressions")]
    Assignment,
    #[error("Postfix-increment as sub-expression")]
    PostIncrementAsSubExpression,
    #[error("Postfix-decrement as sub-expression")]
    PostDecrementAsSubExpression,
    #[error("Prefix-increment as sub-expression")]
    PreIncrementAsSubExpression,
    #[error("Prefix-decrement as sub-expression")]
    PreDecrementAsSubExpression,
    #[error("Bitwise complement as sub-expressions")]
    BitwiseComplementAsSubExpression,
    #[error("Logical not expressions")]
    LogicalNot,
    #[error("Unary plus operator in sub-expressions")]
    UnaryPlusInSubExpression,
    #[error("Unary minus operator in sub-expressions")]
    UnaryMinusInSubExpression,
    #[error("Binary operations")]
    BinaryOp,
    #[error("Ternary conditional expressions")]
    TernaryConditional,
    #[error("Member access expressions")]
    MemberAccess,
    #[error("Method calls")]
    MethodCall,
    #[error("Instance creation expressions")]
    InstanceCreation,
    #[error("Array creation expressions")]
    ArrayCreation,
    #[error("Array access expressions")]
    ArrayAccess,
    #[error("Switch expressions")]
    SwitchExpression,
    #[error("`this` expressions")]
    This,
    #[error("Qualified `this` expressions")]
    QualifiedThis,
    #[error("Class literals")]
    ClassLiteral,
    #[error("Method references")]
    MethodReference,
    #[error("Instance initializer")]
    InstanceInitializer,
    #[error("Static initializer")]
    StaticInitializer,
    #[error("Methods without bodies")]
    NoBodyMethod,
    #[error("Nested classes")]
    NestedClass,
    #[error("Nested interfaces")]
    NestedInterface,
    #[error("Class fields")]
    ClassField,
    #[error("Constructors")]
    Constructor,
    #[error("Compact constructors")]
    CompactConstructor,
    #[error("Empty statements")]
    EmptyStatement,
    #[error("Blocks")]
    Block,
    #[error("Variable declarations")]
    VariableDeclaration,
    #[error("If statements")]
    If,
    #[error("While statements")]
    While,
    #[error("For statements")]
    For,
    #[error("Enhanced for statements")]
    ForEach,
    #[error("Do-while statements")]
    DoWhile,
    #[error("Labeled statements")]
    Labeled,
    #[error("Break statements")]
    Break,
    #[error("Continue statements")]
    Continue,
    #[error("Assert statements")]
    Assert,
    #[error("Return statements")]
    Return,
    #[error("Try statements")]
    Try,
    #[error("Throw statements")]
    Throw,
    #[error("Synchronized statements")]
    Synchronized,
    #[error("Switch statements")]
    SwitchStatement,
    #[error("Yield statements")]
    Yield,
    #[error("Record class declarations")]
    RecordClass,
    #[error("Enum declarations")]
    EnumClass,
    #[error("Interface declarations")]
    Interface,
    #[error("@interface declarations")]
    AnnotationInterface,
}

impl SubError for UnimplementedFeature {}
