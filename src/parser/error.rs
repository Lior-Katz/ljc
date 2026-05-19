use crate::LexicalError;
use crate::file::Span;
use crate::lexer::Symbol;
use crate::parser::Diagnostic;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub type ParseResult<T> = Result<T, Failure>;

pub enum Failure {
    NoProduction,
    Error(Diagnostic),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Lexical(LexicalError),
    #[error("Trailing {0} not allowed here",)]
    MissingElementAfterDelimiter(Symbol),
    #[error("Symbol expected: {0}")]
    SymbolExpected(Symbol),
    #[error("Symbol expected: {0} or {1}")]
    SymbolExpected2(Symbol, Symbol),
    #[error("Expected {0} after {1}")]
    SyntaxExpectedAfter(SyntaxKind, Symbol),
    #[error("Invalid type name")]
    RestrictedTypeName,
    #[error("Expected identifier")]
    IdentifierExpected,
    #[error("Expected block after 'try'\nnote: a block must start with '{{'")]
    MissingTryBlock,
    #[error("Missing class body\nhint: expected '{{'")]
    MissingClassBody,
}

impl Error {
    pub fn at(self, span: Span) -> Diagnostic {
        Diagnostic { span, message: self }
    }
}

impl From<LexicalError> for Error {
    fn from(value: LexicalError) -> Self {
        Error::Lexical(value)
    }
}

impl From<Diagnostic> for Failure {
    fn from(value: Diagnostic) -> Self {
        Failure::Error(value)
    }
}

impl<T> From<(T, Span)> for Diagnostic
where
    T: Into<Error>,
{
    fn from(value: (T, Span)) -> Self {
        value.0.into().at(value.1)
    }
}

#[allow(dead_code)]
pub trait AssertResult {
    fn assert(self, error: Diagnostic) -> Self;
}

impl<T> AssertResult for ParseResult<T> {
    fn assert(self, error: Diagnostic) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(Failure::NoProduction) => Err(Failure::Error(error)),
            Err(Failure::Error(cause)) => Err(Failure::Error(cause)),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxKind {
    Expression,
    Statement,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxKind::Expression => write!(f, "expression"),
            SyntaxKind::Statement => write!(f, "statement"),
        }
    }
}
