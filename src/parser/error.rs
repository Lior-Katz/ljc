use crate::LexicalError;
use crate::lexer::Symbol;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

pub type ParseResult<T> = Result<T, Failure>;

pub enum Failure {
    NoProduction,
    Error(Error),
}

impl From<Error> for Failure {
    fn from(value: Error) -> Self {
        Failure::Error(value)
    }
}

impl From<LexicalError> for Error {
    fn from(value: LexicalError) -> Self {
        Error::Lexical(value)
    }
}

impl From<LexicalError> for Failure {
    fn from(value: LexicalError) -> Self {
        Error::from(value).into()
    }
}

#[allow(dead_code)]
pub trait ResultExtension {
    fn assert(self, error: Error) -> Self;
}

impl<T> ResultExtension for ParseResult<T> {
    fn assert(self, error: Error) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(Failure::NoProduction) => Err(Failure::Error(error)),
            Err(Failure::Error(cause)) => Err(Failure::Error(cause)),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Lexical(LexicalError),
    #[error("Missing element after {0}")]
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
