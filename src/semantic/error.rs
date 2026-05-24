use crate::collections::{AtLeastOne, Multiple, TryCollect};
use crate::semantic::Diagnostic;
use std::fmt::Display;

pub type SemanticResult<T = ()> = Result<T, AtLeastOne<Diagnostic>>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0} not yet supported")]
    Unimplemented(#[from] UnimplementedFeature),
}

trait SubError: Into<Error> + Display {}

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

pub trait Coalesce<T>
where
    Self: IntoIterator<Item = T>,
{
    fn coalesce<F>(&self, op: F) -> SemanticResult
    where
        F: Fn(&T) -> SemanticResult;
}

impl<T> Coalesce<T> for Multiple<T> {
    fn coalesce<F>(&self, op: F) -> SemanticResult
    where
        F: Fn(&T) -> SemanticResult,
    {
        self.into_iter()
            .filter_map(|d| op(d).err())
            .flat_map(IntoIterator::into_iter)
            .try_collect()
            .map_or(Ok(()), Err)
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
    #[error("Postfix-increment expressions")]
    PostIncrement,
    #[error("Postfix-decrement expressions")]
    PostDecrement,
    #[error("Prefix-increment expressions")]
    PreIncrement,
    #[error("Prefix-decrement expressions")]
    PreDecrement,
    #[error("Bitwise complement expressions")]
    BitwiseComplement,
    #[error("Logical not expressions")]
    LogicalNot,
    #[error("Unary plus operator")]
    UnaryPlus,
    #[error("Unary minus operator")]
    UnaryMinus,
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
